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
    other: AtomicUsize,
}

impl FilterCounterState {
    pub fn new() -> Self {
        Self {
            background_video: AtomicUsize::new(0),
            background_image: AtomicUsize::new(0),
            storyboard: AtomicUsize::new(0),
            skin_element: AtomicUsize::new(0),
            hitsound: AtomicUsize::new(0),
            other: AtomicUsize::new(0),
        }
    }

    fn get_counter(&self, file_type: FileType) -> &AtomicUsize {
        match file_type {
            FileType::BackgroundVideo => &self.background_video,
            FileType::BackgroundImage => &self.background_image,
            FileType::Storyboard => &self.storyboard,
            FileType::SkinElement => &self.skin_element,
            FileType::Hitsound => &self.hitsound,
            FileType::Other => &self.other,
        }
    }

    pub fn increment(&self, file_type: FileType) {
        self.get_counter(file_type).fetch_add(1, Ordering::Relaxed);
    }

    fn reset_counter(&self, counter: &AtomicUsize) -> usize {
        counter.swap(0, Ordering::Relaxed)
    }

    pub fn get_and_reset(&self) -> HashMap<&'static str, usize> {
        let mut counts = HashMap::new();
        counts.insert(
            "background_video",
            self.reset_counter(&self.background_video),
        );
        counts.insert(
            "background_image",
            self.reset_counter(&self.background_image),
        );
        counts.insert("storyboard", self.reset_counter(&self.storyboard));
        counts.insert("skin_element", self.reset_counter(&self.skin_element));
        counts.insert("hitsound", self.reset_counter(&self.hitsound));
        counts.insert("other", self.reset_counter(&self.other));
        counts
    }
}
