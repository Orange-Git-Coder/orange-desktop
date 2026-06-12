mod menu;
mod events;

use tauri::tray::TrayIconBuilder;

pub fn setup(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let menu = menu::build_menu(app)?;
    let handle = app.handle().clone();
    let icon = app.default_window_icon().cloned().unwrap();

    TrayIconBuilder::new()
        .icon(icon)
        .tooltip("orange-desktop")
        .menu(&menu)
        .on_menu_event(events::handle_menu_event)
        .on_tray_icon_event(move |_tray, event| {
            events::handle_tray_icon_event(&handle, event);
        })
        .build(app)?;

    Ok(())
}