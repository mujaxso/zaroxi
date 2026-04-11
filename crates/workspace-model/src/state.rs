use std::collections::HashMap;
use std::path::PathBuf;

use uuid::Uuid;

use core_types::workspace::{BufferId, DirectoryEntry};
use editor_core::Document;

#[derive(Debug)]
pub struct WorkspaceState {
    root_path: PathBuf,
    file_tree: Vec<DirectoryEntry>,
    open_buffers: HashMap<BufferId, OpenBuffer>,
    path_to_buffer_id: HashMap<PathBuf, BufferId>,
    active_buffer_id: Option<BufferId>,
}

#[derive(Debug)]
pub struct OpenBuffer {
    pub buffer_id: BufferId,
    pub path: PathBuf,
    pub document: Document,
}

impl WorkspaceState {
    pub fn new(root_path: impl Into<PathBuf>) -> Self {
        Self {
            root_path: root_path.into(),
            file_tree: Vec::new(),
            open_buffers: HashMap::new(),
            path_to_buffer_id: HashMap::new(),
            active_buffer_id: None,
        }
    }

    pub fn set_workspace_root(&mut self, path: impl Into<PathBuf>) {
        self.root_path = path.into();
    }

    pub fn set_file_tree(&mut self, entries: Vec<DirectoryEntry>) {
        self.file_tree = entries;
    }

    pub fn file_tree(&self) -> &[DirectoryEntry] {
        &self.file_tree
    }

    pub fn open_buffer(&mut self, path: impl Into<PathBuf>, content: String) -> BufferId {
        let path = path.into();
        if let Some(&buffer_id) = self.path_to_buffer_id.get(&path) {
            // Buffer already open
            self.active_buffer_id = Some(buffer_id);
            return buffer_id;
        }

        let buffer_id = BufferId(Uuid::new_v4());
        let document = Document::from_text(&content);
        
        let open_buffer = OpenBuffer {
            buffer_id,
            path: path.clone(),
            document,
        };
        
        self.open_buffers.insert(buffer_id, open_buffer);
        self.path_to_buffer_id.insert(path, buffer_id);
        self.active_buffer_id = Some(buffer_id);
        
        buffer_id
    }

    pub fn set_active_buffer(&mut self, buffer_id: BufferId) -> bool {
        if self.open_buffers.contains_key(&buffer_id) {
            self.active_buffer_id = Some(buffer_id);
            true
        } else {
            false
        }
    }

    pub fn active_buffer(&self) -> Option<&Document> {
        self.active_buffer_id
            .and_then(|id| self.open_buffers.get(&id))
            .map(|open_buffer| &open_buffer.document)
    }

    pub fn active_buffer_mut(&mut self) -> Option<&mut Document> {
        self.active_buffer_id
            .and_then(move |id| self.open_buffers.get_mut(&id))
            .map(|open_buffer| &mut open_buffer.document)
    }

    pub fn active_buffer_path(&self) -> Option<PathBuf> {
        self.active_buffer_id
            .and_then(|id| self.open_buffers.get(&id))
            .map(|open_buffer| open_buffer.path.clone())
    }

    pub fn save_active_buffer(&mut self) -> Option<(PathBuf, String)> {
        if let Some(buffer_id) = self.active_buffer_id {
            if let Some(open_buffer) = self.open_buffers.get_mut(&buffer_id) {
                open_buffer.document.mark_saved();
                let content = open_buffer.document.text();
                let path = open_buffer.path.clone();
                return Some((path, content));
            }
        }
        None
    }
}
