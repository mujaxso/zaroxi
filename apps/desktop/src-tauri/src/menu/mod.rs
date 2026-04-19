use tauri::menu::{Menu, MenuItem, Submenu, CustomMenuItem};

pub fn create_app_menu() -> Menu {
    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("open_workspace", "Open Workspace").accelerator("CmdOrCtrl+O"))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrCtrl+Q")),
    );
    
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste),
    );
    
    Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
}
