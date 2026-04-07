mod app;
mod bootstrap;
mod commands;
mod ui;

use std::env;
use std::sync::{Arc, Mutex};

use eframe::egui;
use workspace_daemon::files;
use workspace_model::state::WorkspaceState;

struct NeoteApp {
    workspace_path: String,
    workspace_state: Arc<Mutex<WorkspaceState>>,
    file_entries: Vec<core_types::workspace::DirectoryEntry>,
    selected_file_index: Option<usize>,
    editor_text: String,
    dirty: bool,
}

impl NeoteApp {
    fn new(workspace_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let entries = files::list_directory(&workspace_path)?;
        let workspace_state = Arc::new(Mutex::new(WorkspaceState::new(&workspace_path)));
        workspace_state.lock().unwrap().set_file_tree(entries.clone());
        
        Ok(Self {
            workspace_path,
            workspace_state,
            file_entries: entries,
            selected_file_index: None,
            editor_text: String::new(),
            dirty: false,
        })
    }

    fn open_file(&mut self, index: usize) {
        if index < self.file_entries.len() {
            let entry = &self.file_entries[index];
            if !entry.is_dir {
                match files::read_file(&entry.path) {
                    Ok(content) => {
                        let mut state = self.workspace_state.lock().unwrap();
                        state.open_buffer(&entry.path, content.clone());
                        self.editor_text = content;
                        self.selected_file_index = Some(index);
                        self.dirty = false;
                    }
                    Err(e) => {
                        eprintln!("Failed to read file: {}", e);
                    }
                }
            }
        }
    }

    fn save_current_file(&mut self) {
        let state = self.workspace_state.lock().unwrap();
        if let Some((path, _)) = state.save_active_buffer() {
            match files::write_file(&path.to_string_lossy(), &self.editor_text) {
                Ok(_) => {
                    self.dirty = false;
                }
                Err(e) => {
                    eprintln!("Failed to save file: {}", e);
                }
            }
        }
    }
}

impl eframe::App for NeoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Neote - Step 1");
            ui.horizontal(|ui| {
                ui.label(format!("Workspace: {}", self.workspace_path));
                if ui.button("Refresh").clicked() {
                    if let Ok(entries) = files::list_directory(&self.workspace_path) {
                        self.file_entries = entries;
                        self.workspace_state.lock().unwrap().set_file_tree(self.file_entries.clone());
                    }
                }
            });

            ui.separator();

            ui.columns(2, |columns| {
                // Left column: file list
                columns[0].vertical(|ui| {
                    ui.heading("Files");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, entry) in self.file_entries.iter().enumerate() {
                            let label = if entry.is_dir {
                                format!("📁 {}", entry.name)
                            } else {
                                format!("📄 {}", entry.name)
                            };
                            if ui.selectable_label(self.selected_file_index == Some(i), label).clicked() && !entry.is_dir {
                                self.open_file(i);
                            }
                        }
                    });
                });

                // Right column: editor
                columns[1].vertical(|ui| {
                    ui.horizontal(|ui| {
                        if let Some(index) = self.selected_file_index {
                            let entry = &self.file_entries[index];
                            ui.heading(&entry.name);
                            if self.dirty {
                                ui.label("(modified)");
                            }
                        } else {
                            ui.heading("No file selected");
                        }
                        
                        if ui.button("Save").clicked() {
                            self.save_current_file();
                        }
                    });
                    
                    ui.separator();
                    
                    let mut state = self.workspace_state.lock().unwrap();
                    if let Some(buffer) = state.active_buffer_mut() {
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut self.editor_text)
                                .desired_rows(20)
                                .desired_width(f32::INFINITY)
                        );
                        
                        if response.changed() {
                            buffer.replace_all(self.editor_text.clone());
                            self.dirty = buffer.is_dirty();
                        }
                    } else {
                        ui.label("Select a file to edit");
                    }
                });
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Status:");
                if self.dirty {
                    ui.label("File has unsaved changes");
                } else {
                    ui.label("All changes saved");
                }
            });
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <workspace-path>", args[0]);
        println!("Example: {} /tmp/test_workspace", args[0]);
        return Ok(());
    }
    
    let workspace_path = args[1].clone();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Neote - Step 1"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Neote",
        options,
        Box::new(|_cc| {
            match NeoteApp::new(workspace_path) {
                Ok(app) => Box::new(app),
                Err(e) => {
                    eprintln!("Failed to initialize app: {}", e);
                    std::process::exit(1);
                }
            }
        }),
    ).map_err(|e| e.into())
}
