//! 系统托盘模块

mod events;
mod menu;

use tauri::tray::TrayIconBuilder;

/// 初始化系统托盘
pub fn setup(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let menu = menu::build_menu(app)?;
    let handle = app.handle().clone();

    // 优先使用 app-icon，回退到 icon.ico
    let icon = app
        .default_window_icon()
        .cloned()
        .unwrap();

    TrayIconBuilder::new()
        .icon(icon)
        .tooltip("Orange Desktop")
        .menu(&menu)
        .on_menu_event(events::handle_menu_event)
        .on_tray_icon_event(move |_tray, event| {
            events::handle_tray_icon_event(&handle, event);
        })
        .build(app)?;

    Ok(())
}