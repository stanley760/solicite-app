use tauri::{App, Manager};
/// setup
#[allow(unused_imports)]
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use window_vibrancy::{self, NSVisualEffectMaterial};
    let win = app.get_webview_window("main").unwrap();

    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // 仅在 windows 下执行
    #[cfg(target_os = "windows")]
    window_vibrancy::apply_blur(&win, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    Ok(())
}
