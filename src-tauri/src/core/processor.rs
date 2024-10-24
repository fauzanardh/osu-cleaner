use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use ahash::AHashMap;
use anyhow::Result;
use rayon::prelude::*;
use tauri::{AppHandle, Emitter};
use walkdir::WalkDir;

use crate::core::consts::{deletion, scanner, status_values};
use crate::core::counters::{CommonCounterState, FilterCounterState};
use crate::core::models::{
    CategoryDataResponse, CategoryDetailSimple, CategorySummaryResponse, FileInfo, FileType,
    ScanContext, ScanResult,
};
use crate::core::patterns::FilePatterns;

pub struct FileProcessor {
    emit_interval: u64,
    last_emit: Arc<AtomicU64>,
    scan_counters: Arc<CommonCounterState>,
    parse_counters: Arc<CommonCounterState>,
    filter_counters: Arc<FilterCounterState>,
    scan_result: Arc<Mutex<Option<ScanResult>>>,
}

impl FileProcessor {
    pub fn new() -> Self {
        Self {
            scan_result: Arc::new(Mutex::new(None)),
            last_emit: Arc::new(AtomicU64::new(0)),
            emit_interval: 1_000_000 / 20, // 20 times per second
            scan_counters: Arc::new(CommonCounterState::new()),
            parse_counters: Arc::new(CommonCounterState::new()),
            filter_counters: Arc::new(FilterCounterState::new()),
        }
    }

    #[inline]
    fn try_emit_scan_counts(&self, app: &AppHandle, force: bool) {
        if force {
            let count = self.scan_counters.get_and_reset();
            if count > 0 {
                app.emit(scanner::SCAN_COUNTS, count).unwrap();
            }
        } else {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;

            let last = self.last_emit.load(Ordering::Relaxed);
            if now >= last + self.emit_interval {
                if self
                    .last_emit
                    .compare_exchange(last, now, Ordering::Release, Ordering::Relaxed)
                    .is_ok()
                {
                    let count = self.scan_counters.get_and_reset();
                    if count > 0 {
                        app.emit(scanner::SCAN_COUNTS, count).unwrap();
                    }
                }
            }
        }
    }

    #[inline]
    fn try_emit_parse_counts(&self, app: &AppHandle, force: bool) {
        if force {
            let count = self.parse_counters.get_and_reset();
            if count > 0 {
                app.emit(scanner::PARSE_COUNTS, count).unwrap();
            }
        } else {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;

            let last = self.last_emit.load(Ordering::Relaxed);
            if now >= last + self.emit_interval {
                if self
                    .last_emit
                    .compare_exchange(last, now, Ordering::Release, Ordering::Relaxed)
                    .is_ok()
                {
                    let count = self.parse_counters.get_and_reset();
                    if count > 0 {
                        app.emit(scanner::PARSE_COUNTS, count).unwrap();
                    }
                }
            }
        }
    }

    #[inline]
    fn try_emit_filter_counts(&self, app: &AppHandle, force: bool) {
        if force {
            let counts = self.filter_counters.get_and_reset();
            if counts.values().any(|&count| count > 0) {
                app.emit(scanner::FILTER_COUNTS, counts).unwrap();
            }
        } else {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;

            let last = self.last_emit.load(Ordering::Relaxed);
            if now >= last + self.emit_interval {
                if self
                    .last_emit
                    .compare_exchange(last, now, Ordering::Release, Ordering::Relaxed)
                    .is_ok()
                {
                    let counts = self.filter_counters.get_and_reset();
                    if counts.values().any(|&count| count > 0) {
                        app.emit(scanner::FILTER_COUNTS, counts).unwrap();
                    }
                }
            }
        }
    }

    fn parse_file<F>(&self, path: &Path, context: &mut ScanContext, parser: F) -> Result<()>
    where
        F: Fn(&str, &Path, &mut ScanContext) -> Option<()>,
    {
        let file = File::open(path)?;
        let reader = BufReader::with_capacity(128 * 1024, file); // 128 KB buffer
        let parent = path.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("failed to get parent directory of {:?}", path),
            )
        })?;

        for line in reader.lines() {
            if let Ok(line) = line {
                parser(&line, parent, context);
            }
        }
        Ok(())
    }

    fn parse_osu_file(&self, path: &Path, context: &mut ScanContext) -> Result<()> {
        let in_events = RefCell::new(false);
        self.parse_file(path, context, |line, parent, context| match line.trim() {
            "[Events]" => {
                *in_events.borrow_mut() = true;
                None
            }
            _ if !*in_events.borrow() => None,
            _ if line.starts_with('[') => {
                *in_events.borrow_mut() = false;
                None
            }
            _ if line.starts_with("0,0,\"") || line.starts_with("Video,") => {
                self.extract_quoted_path(line)
                    .map(|file_path| context.backgrounds.insert(parent.join(file_path)));
                None
            }
            _ => None,
        })
    }

    fn parse_storyboard_file(&self, path: &Path, context: &mut ScanContext) -> Result<()> {
        self.parse_file(path, context, |line, parent, context| {
            if line.starts_with("Sprite,") {
                self.extract_quoted_path(line).map(|sprite_path| {
                    context.storyboard_elements.insert(parent.join(sprite_path))
                });
            }
            None
        })
    }

    fn extract_quoted_path<'a>(&self, line: &'a str) -> Option<&'a str> {
        let start = line.find('"')?;
        let end = line[start + 1..].find('"')?;
        Some(&line[start + 1..start + 1 + end])
    }

    pub fn scan_directory(&self, app: Arc<AppHandle>, path: &Path) -> Result<()> {
        println!("Scanning requested for {:?}", path);

        app.emit(scanner::STATUS, status_values::SCAN_START)
            .unwrap();
        let entries: Vec<_> = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| {
                if let Ok(e) = e {
                    if e.file_type().is_file() {
                        self.scan_counters.increment();
                        self.try_emit_scan_counts(&app, false);
                        return Some(e);
                    }
                }
                None
            })
            .collect();
        self.try_emit_scan_counts(&app, true); // Flush remaining counts

        app.emit(scanner::STATUS, status_values::PARSE_START)
            .unwrap();
        let scan_context = entries
            .par_iter()
            .filter(|entry| {
                if let Some(ext) = entry.path().extension().and_then(|ext| ext.to_str()) {
                    if ext == "osu" || ext == "osb" {
                        self.parse_counters.increment();
                        self.try_emit_parse_counts(&app, false);
                        return true;
                    }
                }
                false
            })
            .fold(
                || ScanContext::default(),
                |mut context, entry| {
                    if let Some(ext) = entry.path().extension().and_then(|ext| ext.to_str()) {
                        let _ = match ext {
                            "osu" => self.parse_osu_file(entry.path(), &mut context),
                            "osb" => self.parse_storyboard_file(entry.path(), &mut context),
                            _ => Ok(()),
                        };
                    }
                    context
                },
            )
            .reduce(
                || ScanContext::default(),
                |mut a, b| {
                    a.backgrounds.extend(b.backgrounds);
                    a.storyboard_elements.extend(b.storyboard_elements);
                    a
                },
            );
        self.try_emit_parse_counts(&app, true); // Flush remaining counts

        app.emit(scanner::STATUS, status_values::FILTER_START)
            .unwrap();
        let patterns = FilePatterns::new();
        let scan_result = entries
            .par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                let file_type =
                    self.categorize_file(&patterns, Arc::clone(&app), path, &scan_context);

                if file_type == FileType::Other {
                    return None;
                }

                entry.metadata().ok().map(|metadata| {
                    let size = metadata.len();
                    (
                        file_type,
                        FileInfo {
                            path: path.to_owned(),
                            size,
                        },
                        size,
                    )
                })
            })
            .fold(
                ScanResult::default,
                |mut result, (file_type, file_info, size)| {
                    result.total_size += size;
                    result.files.entry(file_type).or_default().push(file_info);
                    result
                },
            )
            .reduce(ScanResult::default, |mut a, b| {
                a.total_size += b.total_size;
                for (file_type, files) in b.files {
                    a.files.entry(file_type).or_default().extend(files);
                }
                a
            });
        self.try_emit_filter_counts(&app, true); // Flush remaining counts

        *self.scan_result.lock().unwrap() = Some(scan_result);
        Ok(())
    }

    fn categorize_file(
        &self,
        patterns: &FilePatterns,
        app: Arc<AppHandle>,
        path: &Path,
        context: &ScanContext,
    ) -> FileType {
        let file_type = if patterns.is_video(path) {
            FileType::BackgroundVideo
        } else if context.backgrounds.contains(path) {
            FileType::BackgroundImage
        } else if context.storyboard_elements.contains(path) || patterns.is_storyboard_file(path) {
            FileType::Storyboard
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if patterns.is_skin_element(name) {
                FileType::SkinElement
            } else if patterns.is_hitsound(name) {
                FileType::Hitsound
            } else {
                FileType::Other
            }
        } else {
            FileType::Other
        };

        self.filter_counters.increment(file_type);
        self.try_emit_filter_counts(&app, false);
        file_type
    }

    pub fn get_category_summary(&self) -> Option<CategorySummaryResponse> {
        let binding = self.get_scan_result();
        let scan_result = binding.lock().unwrap();
        let scan_result = match &*scan_result {
            Some(scan_result) => scan_result,
            None => return None,
        };

        let store: AHashMap<_, _> = scan_result
            .files
            .iter()
            .map(|(file_type, files)| {
                let detail = CategoryDetailSimple {
                    total_size: files.iter().map(|file| file.size).sum(),
                    total_count: files.len() as u64,
                };
                (*file_type, detail)
            })
            .collect();

        Some(CategorySummaryResponse {
            background_video: store
                .get(&FileType::BackgroundVideo)
                .cloned()
                .unwrap_or_default(),
            background_image: store
                .get(&FileType::BackgroundImage)
                .cloned()
                .unwrap_or_default(),
            storyboard: store
                .get(&FileType::Storyboard)
                .cloned()
                .unwrap_or_default(),
            skin_element: store
                .get(&FileType::SkinElement)
                .cloned()
                .unwrap_or_default(),
            hitsound: store.get(&FileType::Hitsound).cloned().unwrap_or_default(),
        })
    }

    pub fn get_category_data(&self, category: &str) -> Option<CategoryDataResponse> {
        let binding = self.get_scan_result();
        let scan_result = binding.lock().unwrap();
        let scan_result = match &*scan_result {
            Some(scan_result) => scan_result,
            None => return None,
        };

        let files = scan_result.files.get(&match category {
            "background_video" => FileType::BackgroundVideo,
            "background_image" => FileType::BackgroundImage,
            "storyboard" => FileType::Storyboard,
            "skin_element" => FileType::SkinElement,
            "hitsound" => FileType::Hitsound,
            _ => return None,
        });

        Some(CategoryDataResponse {
            files: files.cloned().unwrap_or_default(),
        })
    }

    pub fn delete_files(&self, app: Arc<AppHandle>, categories: Vec<&str>) -> Result<()> {
        let scan_result = self.get_scan_result();
        let mut scan_result = scan_result.lock().unwrap();
        let scan_result = match &mut *scan_result {
            Some(scan_result) => scan_result,
            None => return Ok(()),
        };

        categories.iter().for_each(|category| {
            let file_type = match *category {
                "background_video" => FileType::BackgroundVideo,
                "background_image" => FileType::BackgroundImage,
                "storyboard" => FileType::Storyboard,
                "skin_element" => FileType::SkinElement,
                "hitsound" => FileType::Hitsound,
                _ => return,
            };

            println!(
                "Deleting files for category {:?}: {:?}",
                category, file_type
            );

            app.emit(deletion::CATEGORY_START, category).unwrap();
            if let Some(files) = scan_result.files.get_mut(&file_type) {
                files.par_iter().for_each(|file| {
                    if let Err(e) = std::fs::remove_file(&file.path) {
                        eprintln!("Failed to delete file {:?}: {}", file.path, e);
                    }
                });
            }
            app.emit(deletion::CATEGORY_COMPLETE, category).unwrap();
        });

        Ok(())
    }

    fn get_scan_result(&self) -> Arc<Mutex<Option<ScanResult>>> {
        Arc::clone(&self.scan_result)
    }
}
