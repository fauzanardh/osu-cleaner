use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::core::models::FileType;

pub struct CommonCounterState {
    count: AtomicUsize,
}

impl CommonCounterState {
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_and_reset(&self) -> usize {
        self.count.swap(0, Ordering::Relaxed)
    }
}

pub struct FilterCounterState {
    background_video: AtomicUsize,
    background_image: AtomicUsize,
    storyboard: AtomicUsize,
    skin_element: AtomicUsize,
    hitsound: AtomicUsize,
    unknown: AtomicUsize,
}

impl FilterCounterState {
    pub fn new() -> Self {
        Self {
            background_video: AtomicUsize::new(0),
            background_image: AtomicUsize::new(0),
            storyboard: AtomicUsize::new(0),
            skin_element: AtomicUsize::new(0),
            hitsound: AtomicUsize::new(0),
            unknown: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self, file_type: FileType) {
        match file_type {
            FileType::BackgroundVideo => self.background_video.fetch_add(1, Ordering::Relaxed),
            FileType::BackgroundImage => self.background_image.fetch_add(1, Ordering::Relaxed),
            FileType::Storyboard => self.storyboard.fetch_add(1, Ordering::Relaxed),
            FileType::SkinElement => self.skin_element.fetch_add(1, Ordering::Relaxed),
            FileType::Hitsound => self.hitsound.fetch_add(1, Ordering::Relaxed),
            FileType::Other => self.unknown.fetch_add(1, Ordering::Relaxed),
        };
    }

    pub fn get_and_reset(&self) -> HashMap<&'static str, usize> {
        let mut counts = HashMap::new();
        counts.insert(
            "background_video",
            self.background_video.swap(0, Ordering::Relaxed),
        );
        counts.insert(
            "background_image",
            self.background_image.swap(0, Ordering::Relaxed),
        );
        counts.insert("storyboard", self.storyboard.swap(0, Ordering::Relaxed));
        counts.insert("skin_element", self.skin_element.swap(0, Ordering::Relaxed));
        counts.insert("hitsound", self.hitsound.swap(0, Ordering::Relaxed));
        counts.insert("other", self.unknown.swap(0, Ordering::Relaxed));
        counts
    }
}
