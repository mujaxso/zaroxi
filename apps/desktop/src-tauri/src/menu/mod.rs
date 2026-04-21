use tauri::menu::{Menu, Submenu, PredefinedMenuItem, MenuItem};

pub fn create_app_menu<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<Menu<R>> {
    let open_item = MenuItem::with_id(app, "open_workspace", "Open Workspace", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "open_settings", "Settings", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    
    let file_menu = Submenu::with_items(
        app,
        "File",
        true,
        &[
            &open_item,
            &PredefinedMenuItem::separator(app)?,
            &settings_item,
            &PredefinedMenuItem::separator(app)?,
            &quit_item,
        ],
    )?;
    
    let undo_item = PredefinedMenuItem::undo(app, None)?;
    let redo_item = PredefinedMenuItem::redo(app, None)?;
    let cut_item = PredefinedMenuItem::cut(app, None)?;
    let copy_item = PredefinedMenuItem::copy(app, None)?;
    let paste_item = PredefinedMenuItem::paste(app, None)?;
    
    let edit_menu = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &undo_item,
            &redo_item,
            &PredefinedMenuItem::separator(app)?,
            &cut_item,
            &copy_item,
            &paste_item,
        ],
    )?;
    
    let theme_system = MenuItem::with_id(app, "theme_system", "System", true, None::<&str>)?;
    let theme_light = MenuItem::with_id(app, "theme_light", "Light", true, None::<&str>)?;
    let theme_dark = MenuItem::with_id(app, "theme_dark", "Dark", true, None::<&str>)?;
    
    let theme_menu = Submenu::with_items(
        app,
        "Theme",
        true,
        &[
            &theme_system,
            &theme_light,
            &theme_dark,
        ],
    )?;
    
    let menu = Menu::with_items(app, &[&file_menu, &edit_menu, &theme_menu])?;
    
    Ok(menu)
}
