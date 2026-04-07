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

        // Dialogs
        let mut open_workspace_dialog = self.open_workspace_dialog;
        let mut workspace_path_input = std::mem::take(&mut self.workspace_path_input);
        
        if open_workspace_dialog {
            egui::Window::new("Open Workspace")
                .open(&mut open_workspace_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter workspace path:");
                    ui.text_edit_singleline(&mut workspace_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            open_workspace_dialog = false;
                            workspace_path_input.clear();
                        }
                        if ui.button("Open").clicked() {
                            if !workspace_path_input.is_empty() {
                                match app.open_workspace(workspace_path_input.clone()) {
                                    Ok(_) => {
                                        open_workspace_dialog = false;
                                        workspace_path_input.clear();
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to open workspace: {}", e);
                                    }
                                }
                            }
                        }
                    });
                });
        }
        self.open_workspace_dialog = open_workspace_dialog;
        self.workspace_path_input = workspace_path_input;

        // Create file dialog
        let mut create_file_dialog = self.create_file_dialog;
        let mut create_file_path_input = std::mem::take(&mut self.create_file_path_input);
        
        if create_file_dialog {
            egui::Window::new("Create File")
                .open(&mut create_file_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter file path (relative to workspace):");
                    ui.text_edit_singleline(&mut create_file_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            create_file_dialog = false;
                            create_file_path_input.clear();
                        }
                        if ui.button("Create").clicked() {
                            if !create_file_path_input.is_empty() {
                                let full_path = if !app.workspace_path.is_empty() {
                                    format!("{}/{}", app.workspace_path, create_file_path_input)
                                } else {
                                    create_file_path_input.clone()
                                };
                                match app.create_file(full_path) {
                                    Ok(_) => {
                                        create_file_dialog = false;
                                        create_file_path_input.clear();
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to create file: {}", e);
                                    }
                                }
                            }
                        }
                    });
                });
        }
        self.create_file_dialog = create_file_dialog;
        self.create_file_path_input = create_file_path_input;

        // Delete file dialog
        let mut delete_file_dialog = self.delete_file_dialog;
        let mut delete_file_path_input = std::mem::take(&mut self.delete_file_path_input);
        
        if delete_file_dialog {
            egui::Window::new("Delete File")
                .open(&mut delete_file_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter file path to delete:");
                    ui.text_edit_singleline(&mut delete_file_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            delete_file_dialog = false;
                            delete_file_path_input.clear();
                        }
                        if ui.button("Delete").clicked() {
                            if !delete_file_path_input.is_empty() {
                                let full_path = if !app.workspace_path.is_empty() {
                                    format!("{}/{}", app.workspace_path, delete_file_path_input)
                                } else {
                                    delete_file_path_input.clone()
                                };
                                match app.delete_file(full_path) {
                                    Ok(_) => {
                                        delete_file_dialog = false;
                                        delete_file_path_input.clear();
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to delete file: {}", e);
                                    }
                                }
                            }
                        }
                    });
                });
        }
        self.delete_file_dialog = delete_file_dialog;
        self.delete_file_path_input = delete_file_path_input;

        // Open file dialog
        let mut open_file_dialog = self.open_file_dialog;
        let mut open_file_path_input = std::mem::take(&mut self.open_file_path_input);
        
        if open_file_dialog {
            egui::Window::new("Open File")
                .open(&mut open_file_dialog)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter file path to open:");
                    ui.text_edit_singleline(&mut open_file_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            open_file_dialog = false;
                            open_file_path_input.clear();
                        }
                        if ui.button("Open").clicked() {
                            if !open_file_path_input.is_empty() {
                                let full_path = if !app.workspace_path.is_empty() {
                                    format!("{}/{}", app.workspace_path, open_file_path_input)
                                } else {
                                    open_file_path_input.clone()
                                };
                                // Find the index of the file in file_entries
                                if let Some(index) = app.file_entries.iter().position(|entry| entry.path == full_path) {
                                    app.open_file(index);
                                    open_file_dialog = false;
                                    open_file_path_input.clear();
                                } else {
                                    eprintln!("File not found in workspace");
                                }
                            }
                        }
                    });
                });
        }
        self.open_file_dialog = open_file_dialog;
        self.open_file_path_input = open_file_path_input;
    }
}
