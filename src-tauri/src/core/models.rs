use std::path::PathBuf;

use ahash::AHashMap;
use ahash::AHashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileType {
    BackgroundVideo,
    BackgroundImage,
    Storyboard,
    Hitsound,
    SkinElement,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug, Default)]
pub struct ScanResult {
    pub total_size: u64,
    pub files: AHashMap<FileType, Vec<FileInfo>>,
}

#[derive(Default)]
pub struct ScanContext {
    pub backgrounds: AHashSet<PathBuf>,
    pub storyboard_elements: AHashSet<PathBuf>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CategoryDetailSimple {
    pub total_size: u64,
    pub total_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummaryResponse {
    pub background_video: CategoryDetailSimple,
    pub background_image: CategoryDetailSimple,
    pub storyboard: CategoryDetailSimple,
    pub skin_element: CategoryDetailSimple,
    pub hitsound: CategoryDetailSimple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDataResponse {
    pub files: Vec<FileInfo>,
}
