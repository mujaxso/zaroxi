//! Workspace model for Neote.
//!
//! Defines the data structures and operations for modeling the workspace,
//! including file trees, open editors, project graphs, snapshots, and
//! overall workspace state.

pub mod file_tree;
pub mod open_editors;
pub mod project_graph;
pub mod snapshots;
pub mod workspace;
pub mod state;

#[cfg(test)]
mod tests {
    use super::state::WorkspaceState;
    use core_types::workspace::DirectoryEntry;

    #[test]
    fn test_workspace_state() {
        let mut state = WorkspaceState::new("/test/path");
        
        // Test setting file tree
        let entries = vec![
            DirectoryEntry {
                path: "/test/path/file1.txt".to_string(),
                name: "file1.txt".to_string(),
                is_dir: false,
            },
            DirectoryEntry {
                path: "/test/path/dir".to_string(),
                name: "dir".to_string(),
                is_dir: true,
            },
        ];
        state.set_file_tree(entries);
        assert_eq!(state.file_tree().len(), 2);
        
        // Test opening a buffer
        let buffer_id = state.open_buffer("/test/path/file1.txt", "Hello, world!".to_string());
        assert!(state.active_buffer().is_some());
        assert_eq!(state.active_buffer().unwrap().text(), "Hello, world!");
        
        // Test modifying buffer
        if let Some(buffer) = state.active_buffer_mut() {
            buffer.replace_all("Modified content");
            assert_eq!(buffer.text(), "Modified content");
            assert!(buffer.is_dirty());
        }
        
        // Test saving
        let saved = state.save_active_buffer();
        assert!(saved.is_some());
        let (path, content) = saved.unwrap();
        assert_eq!(path.to_string_lossy(), "/test/path/file1.txt");
        assert_eq!(content, "Modified content");
        
        // Buffer should no longer be dirty after saving
        if let Some(buffer) = state.active_buffer() {
            assert!(!buffer.is_dirty());
        }
    }
}
