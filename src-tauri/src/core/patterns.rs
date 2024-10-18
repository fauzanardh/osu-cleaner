use std::path::Path;

use ahash::AHashSet;

#[derive(Default, Clone)]
pub struct FilePatterns {
    pub video_extensions: AHashSet<&'static str>,
    pub skin_patterns: AHashSet<&'static str>,
    pub hitsound_patterns: AHashSet<&'static str>,
}

impl FilePatterns {
    pub fn new() -> Self {
        let video_extensions: AHashSet<_> = [
            "3gp", "avi", "flv", "m4v", "mkv", "mkv", "mov", "mp4", "mpeg", "mpg", "webm", "wmv",
        ]
        .iter()
        .cloned()
        .collect();
        let skin_patterns: AHashSet<_> = [
            "applause",
            "approachcircle",
            "button-",
            "combobreak",
            "comboburst",
            "count",
            "cursor",
            "default-",
            "failsound",
            "followpoint",
            "fruit-",
            "go.png",
            "go@2x.png",
            "gos.png",
            "gos@2x.png",
            "hit0",
            "hit100",
            "hit300",
            "hit50",
            "hitcircle",
            "inputoverlay-",
            "lighting.png",
            "lighting@2x.png",
            "mania-",
            "menu.",
            "menu-back",
            "particle100",
            "particle300",
            "particle50",
            "pause-",
            "pippidon",
            "play-",
            "ranking-",
            "ready",
            "reversearrow",
            "score-",
            "scorebar-",
            "sectionfail",
            "sectionpass",
            "section-",
            "selection-",
            "sliderb",
            "sliderfollowcircle",
            "sliderscorepoint",
            "spinnerbonus",
            "spinner-",
            "spinnerspin",
            "star.png",
            "star@2x.png",
            "star2.png",
            "star2@2x.png",
            "taiko-",
            "taikobigcircle",
            "taikohitcircle",
        ]
        .iter()
        .cloned()
        .collect();
        let hitsound_patterns: AHashSet<_> =
            ["drum-", "normal-", "soft-"].iter().cloned().collect();
        Self {
            video_extensions,
            skin_patterns,
            hitsound_patterns,
        }
    }

    #[inline]
    pub fn is_video(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.video_extensions.contains(ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    #[inline]
    pub fn is_skin_element(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        self.skin_patterns
            .iter()
            .any(|pattern| name_lower.contains(pattern))
    }

    #[inline]
    pub fn is_hitsound(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        self.hitsound_patterns
            .iter()
            .any(|pattern| name_lower.contains(pattern))
    }

    #[inline]
    pub fn is_storyboard_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "osb")
            .unwrap_or(false)
    }
}
