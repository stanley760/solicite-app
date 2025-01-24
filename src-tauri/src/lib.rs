pub mod config;

pub mod repository;

pub mod domain;

pub mod service;

pub mod utils;

pub mod error;

pub mod setup;

pub use config::*;
pub use domain::*;
pub use error::*;
pub use repository::*;
pub use service::*;
pub use setup::*;
pub use utils::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup::init)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handle_excel,
            update_config,
            read_excel_sheet_names,
            read_config_by_type
        ])
        .run(tauri::generate_context!())
        .expect("运行程序过程中发生了错误");
}
