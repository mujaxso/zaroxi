use tauri::menu::{Menu, Submenu, PredefinedMenuItem, CustomMenuItem};

pub fn create_app_menu<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<Menu<R>> {
    let open_item = CustomMenuItem::new("open_workspace".to_string(), "Open Workspace")
        .accelerator("CmdOrCtrl+O");
    let quit_item = CustomMenuItem::new("quit".to_string(), "Quit")
        .accelerator("CmdOrCtrl+Q");
    
    let file_menu = Submenu::with_items(
        app,
        "File",
        true,
        &[
            &open_item,
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
    
    let menu = Menu::new(app)?
        .add_submenu(file_menu)?
        .add_submenu(edit_menu)?;
    
    Ok(menu)
}
