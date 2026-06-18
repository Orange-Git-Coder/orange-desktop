//! 托盘菜单构建

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    App,
};

/// 构建托盘菜单
pub fn build_menu(app: &App) -> Result<tauri::menu::Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let show_hide = MenuItemBuilder::with_id("show_hide", "显示/隐藏").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&show_hide)
        .item(&quit)
        .build()?;

    Ok(menu)
}