use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

use anyhow::Result;
use tauri::{AppHandle, State};

use crate::core::cancel::CancellationToken;
use crate::core::models::{CategoryDataResponse, CategorySummaryResponse};
use crate::core::processor::FileProcessor;

pub struct FileProcessorService {
    file_processor: FileProcessor,
    app: Arc<AppHandle>,
    active_operation: Arc<Mutex<Option<CancellationToken>>>,
}

impl FileProcessorService {
    pub fn new(app: Arc<AppHandle>) -> Self {
        Self {
            file_processor: FileProcessor::new(),
            app,
            active_operation: Arc::new(Mutex::new(None)),
        }
    }

    pub fn cancel_operation(&self) {
        if let Some(token) = self.active_operation.lock().unwrap().as_ref() {
            token.cancel();
        }
    }

    pub fn scan_directory(&self, path: &Path) -> Result<()> {
        let token = CancellationToken::new();
        let token_ref = token.clone();

        *self.active_operation.lock().unwrap() = Some(token);
        let app = self.app.clone();
        let result = self.file_processor.scan_directory(&app, path, token_ref);
        *self.active_operation.lock().unwrap() = None;

        result
    }

    pub fn get_category_summary(&self) -> Option<CategorySummaryResponse> {
        self.file_processor.get_category_summary()
    }

    pub fn get_category_data(&self, category: &str) -> Option<CategoryDataResponse> {
        self.file_processor.get_category_data(category)
    }

    pub fn delete_files(&self, categories: Vec<&str>) -> Result<()> {
        let token = CancellationToken::new();
        let token_ref = token.clone();

        *self.active_operation.lock().unwrap() = Some(token);
        let app = self.app.clone();
        let result = self
            .file_processor
            .delete_files(&app, categories, token_ref);
        *self.active_operation.lock().unwrap() = None;

        result
    }
}

pub struct FileProcessorState(pub RwLock<FileProcessorService>);

#[tauri::command(async)]
pub fn cancel_operation(state: State<'_, FileProcessorState>) {
    let file_processor_service = state.0.read().unwrap();
    file_processor_service.cancel_operation();
}

#[tauri::command(async)]
pub fn scan_directory(path: &str, state: State<'_, FileProcessorState>) -> Result<(), String> {
    let path = Path::new(&path);
    let file_processor_service = state.0.read().unwrap();
    file_processor_service
        .scan_directory(path)
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub fn get_category_summary(
    state: State<'_, FileProcessorState>,
) -> Result<Option<CategorySummaryResponse>, String> {
    let file_processor_service = state.0.read().unwrap();
    Ok(file_processor_service.get_category_summary())
}

#[tauri::command(async)]
pub fn get_category_data(
    category: &str,
    state: State<'_, FileProcessorState>,
) -> Result<Option<CategoryDataResponse>, String> {
    let file_processor_service = state.0.read().unwrap();
    Ok(file_processor_service.get_category_data(category))
}

#[tauri::command(async)]
pub fn delete_files(
    categories: Vec<&str>,
    state: State<'_, FileProcessorState>,
) -> Result<(), String> {
    let file_processor_service = state.0.read().unwrap();
    file_processor_service
        .delete_files(categories)
        .map_err(|e| e.to_string())
}
