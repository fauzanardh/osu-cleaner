// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{Builder, Manager};

mod core;
mod service;

use crate::service::file_processor_service::{FileProcessorService, FileProcessorState};

fn main() {
    Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.manage(FileProcessorState(Mutex::new(FileProcessorService::new(
                Arc::new(app.handle().clone()),
            ))));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            service::file_processor_service::scan_directory,
            service::file_processor_service::get_category_summary,
            service::file_processor_service::get_category_data,
            service::file_processor_service::delete_files
        ])
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
