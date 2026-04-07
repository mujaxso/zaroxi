use eframe::egui;

pub struct Sidebar {
    pub open_workspace_dialog: bool,
    pub create_file_dialog: bool,
    pub delete_file_dialog: bool,
    pub open_file_dialog: bool,
    pub workspace_path_input: String,
    pub create_file_path_input: String,
    pub delete_file_path_input: String,
    pub open_file_path_input: String,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            open_workspace_dialog: false,
            create_file_dialog: false,
            delete_file_dialog: false,
            open_file_dialog: false,
            workspace_path_input: String::new(),
            create_file_path_input: String::new(),
            delete_file_path_input: String::new(),
            open_file_path_input: String::new(),
        }
    }
}

impl Sidebar {
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut crate::NeoteApp) {
        ui.vertical(|ui| {
            ui.heading("Workspace");
            
            if ui.button("📂 Open Workspace").clicked() {
                self.open_workspace_dialog = true;
            }
            
            ui.separator();
            
            ui.heading("File Operations");
            
            if ui.button("📄 Open File").clicked() {
                self.open_file_dialog = true;
            }
            
            if ui.button("➕ Create File").clicked() {
                self.create_file_dialog = true;
            }
            
            if ui.button("🗑️ Delete File").clicked() {
                self.delete_file_dialog = true;
            }
        });

        // Open workspace dialog
        if self.open_workspace_dialog {
            let mut should_close = false;
            let mut should_open = false;
            
            egui::Window::new("Open Workspace")
                .open(&mut self.open_workspace_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter workspace path:");
                    ui.text_edit_singleline(&mut self.workspace_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            should_close = true;
                            self.workspace_path_input.clear();
                        }
                        if ui.button("Open").clicked() {
                            if !self.workspace_path_input.is_empty() {
                                should_open = true;
                            }
                        }
                    });
                });
            
            if should_close {
                self.open_workspace_dialog = false;
                self.workspace_path_input.clear();
            }
            if should_open {
                let path = self.workspace_path_input.clone();
                match app.open_workspace(path) {
                    Ok(_) => {
                        self.open_workspace_dialog = false;
                        self.workspace_path_input.clear();
                    }
                    Err(e) => {
                        eprintln!("Failed to open workspace: {}", e);
                    }
                }
            }
        }

        // Create file dialog
        if self.create_file_dialog {
            let mut should_close = false;
            let mut should_create = false;
            let mut create_path = String::new();
            
            egui::Window::new("Create File")
                .open(&mut self.create_file_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter file path (relative to workspace):");
                    ui.text_edit_singleline(&mut self.create_file_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            should_close = true;
                            self.create_file_path_input.clear();
                        }
                        if ui.button("Create").clicked() {
                            if !self.create_file_path_input.is_empty() {
                                should_create = true;
                                create_path = self.create_file_path_input.clone();
                            }
                        }
                    });
                });
            
            if should_close {
                self.create_file_dialog = false;
                self.create_file_path_input.clear();
            }
            if should_create {
                let full_path = if !app.workspace_path.is_empty() {
                    format!("{}/{}", app.workspace_path, create_path)
                } else {
                    create_path.clone()
                };
                match app.create_file(full_path) {
                    Ok(_) => {
                        self.create_file_dialog = false;
                        self.create_file_path_input.clear();
                    }
                    Err(e) => {
                        eprintln!("Failed to create file: {}", e);
                    }
                }
            }
        }

        // Delete file dialog
        if self.delete_file_dialog {
            let mut should_close = false;
            let mut should_delete = false;
            let mut delete_path = String::new();
            
            egui::Window::new("Delete File")
                .open(&mut self.delete_file_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter file path to delete:");
                    ui.text_edit_singleline(&mut self.delete_file_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            should_close = true;
                            self.delete_file_path_input.clear();
                        }
                        if ui.button("Delete").clicked() {
                            if !self.delete_file_path_input.is_empty() {
                                should_delete = true;
                                delete_path = self.delete_file_path_input.clone();
                            }
                        }
                    });
                });
            
            if should_close {
                self.delete_file_dialog = false;
                self.delete_file_path_input.clear();
            }
            if should_delete {
                let full_path = if !app.workspace_path.is_empty() {
                    format!("{}/{}", app.workspace_path, delete_path)
                } else {
                    delete_path.clone()
                };
                match app.delete_file(full_path) {
                    Ok(_) => {
                        self.delete_file_dialog = false;
                        self.delete_file_path_input.clear();
                    }
                    Err(e) => {
                        eprintln!("Failed to delete file: {}", e);
                    }
                }
            }
        }

        // Open file dialog
        if self.open_file_dialog {
            let mut should_close = false;
            let mut should_open = false;
            let mut open_path = String::new();
            
            egui::Window::new("Open File")
                .open(&mut self.open_file_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter file path to open:");
                    ui.text_edit_singleline(&mut self.open_file_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            should_close = true;
                            self.open_file_path_input.clear();
                        }
                        if ui.button("Open").clicked() {
                            if !self.open_file_path_input.is_empty() {
                                should_open = true;
                                open_path = self.open_file_path_input.clone();
                            }
                        }
                    });
                });
            
            if should_close {
                self.open_file_dialog = false;
                self.open_file_path_input.clear();
            }
            if should_open {
                let full_path = if !app.workspace_path.is_empty() {
                    format!("{}/{}", app.workspace_path, open_path)
                } else {
                    open_path.clone()
                };
                // Find the index of the file in file_entries
                if let Some(index) = app.file_entries.iter().position(|entry| entry.path == full_path) {
                    app.open_file(index);
                    self.open_file_dialog = false;
                    self.open_file_path_input.clear();
                } else {
                    eprintln!("File not found in workspace");
                }
            }
        }
    }
}
