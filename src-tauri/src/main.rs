// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    solicite_app_lib::run();
    info!(">>>>>>>>>>>>>>>>> starting server solicte-app >>>>>>>>>>>>>");
    Ok(())
}
