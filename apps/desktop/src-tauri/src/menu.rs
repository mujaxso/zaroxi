//! Native macOS menu bar built with Tauri’s menu API.
//! This is only active on macOS; on other platforms the custom React menu is used.

use tauri::{
    menu::{Menu, MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder},
    AppHandle, Wry,
};

/// Build and set the application menu (currently only macOS).
///
/// On non‑macOS platforms this function does nothing.
pub fn build_menu(app: &AppHandle) -> tauri::Result<()> {
    if !cfg!(target_os = "macos") {
        return Ok(());
    }

    // ---- File submenu ----
    let file_open = MenuItemBuilder::with_id("file_open", "Open Folder…")
        .accelerator("CmdOrCtrl+O")
        .build(app)?;
    let file_new = MenuItemBuilder::with_id("file_new", "New File")
        .accelerator("CmdOrCtrl+N")
        .build(app)?;
    let file_save = MenuItemBuilder::with_id("file_save", "Save")
        .accelerator("CmdOrCtrl+S")
        .build(app)?;
    let file_save_as = MenuItemBuilder::with_id("file_save_as", "Save As…")
        .accelerator("CmdOrCtrl+Shift+S")
        .build(app)?;
    let file_close = MenuItemBuilder::with_id("file_close", "Close Window")
        .accelerator("CmdOrCtrl+W")
        .build(app)?;

    let file_submenu = SubmenuBuilder::new(app, "File")
        .item(&file_open)
        .item(&file_new)
        .separator()
        .item(&file_save)
        .item(&file_save_as)
        .separator()
        .item(&file_close)
        .build()?;

    // ---- Edit submenu ----
    let edit_undo = PredefinedMenuItem::undo(app, Some("Undo"))?;
    let edit_redo = PredefinedMenuItem::redo(app, Some("Redo"))?;
    let edit_cut = PredefinedMenuItem::cut(app, Some("Cut"))?;
    let edit_copy = PredefinedMenuItem::copy(app, Some("Copy"))?;
    let edit_paste = PredefinedMenuItem::paste(app, Some("Paste"))?;
    let edit_select_all = PredefinedMenuItem::select_all(app, Some("Select All"))?;

    let edit_submenu = SubmenuBuilder::new(app, "Edit")
        .item(&edit_undo)
        .item(&edit_redo)
        .separator()
        .item(&edit_cut)
        .item(&edit_copy)
        .item(&edit_paste)
        .separator()
        .item(&edit_select_all)
        .build()?;

    // ---- View submenu ----
    let view_toggle_sidebar = MenuItemBuilder::with_id("view_toggle_sidebar", "Toggle Sidebar")
        .accelerator("CmdOrCtrl+B")
        .build(app)?;

    let view_submenu = SubmenuBuilder::new(app, "View")
        .item(&view_toggle_sidebar)
        .build()?;

    // ---- Tools submenu ----
    let tools_settings = MenuItemBuilder::with_id("tools_settings", "Settings…")
        .accelerator("CmdOrCtrl+,")
        .build(app)?;

    let tools_submenu = SubmenuBuilder::new(app, "Tools")
        .item(&tools_settings)
        .build()?;

    // ---- Combine everything ----
    let menu = MenuBuilder::new(app)
        .item(&file_submenu)
        .item(&edit_submenu)
        .item(&view_submenu)
        .item(&tools_submenu)
        .build()?;

    app.set_menu(menu)?;

    tracing::info!("Native macOS menu bar registered");

    Ok(())
}
