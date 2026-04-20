use crate::services::workspace_service::{FileEntry, ExplorerTreeNode};
use chrono::{DateTime, Utc};

/// Convert domain workspace to DTO
pub fn domain_workspace_to_dto(
    workspace: &zaroxi_domain_workspace::workspace::Workspace,
) -> crate::commands::workspace::OpenWorkspaceResponse {
    // Count files in the workspace directory
    let file_count = match std::fs::read_dir(&workspace.root_path) {
        Ok(entries) => entries.count(),
        Err(_) => 0,
    };
    
    crate::commands::workspace::OpenWorkspaceResponse {
        workspace_id: workspace.id.to_string(),
        root_path: workspace.root_path.clone(),
        file_count,
    }
}

/// Convert file entry to DTO
pub fn file_entry_to_dto(entry: &FileEntry) -> crate::commands::workspace::DirectoryEntryDto {
    // Format modified time if available
    let modified_str = entry.modified.and_then(|time| {
        let datetime: DateTime<Utc> = time.into();
        Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    });
    
    crate::commands::workspace::DirectoryEntryDto {
        path: entry.path.clone(),
        name: entry.name.clone(),
        is_dir: entry.is_dir,
        file_type: entry.file_type.clone(),
        size: entry.size,
        modified: modified_str,
    }
}

/// Convert file entry to explorer tree node
#[allow(dead_code)]
pub fn file_entry_to_tree_node(entry: &FileEntry) -> ExplorerTreeNode {
    let modified_str = entry.modified.and_then(|time| {
        let datetime: DateTime<Utc> = time.into();
        Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    });
    
    ExplorerTreeNode {
        id: entry.path.clone(),
        path: entry.path.clone(),
        name: entry.name.clone(),
        is_dir: entry.is_dir,
        file_type: entry.file_type.clone(),
        size: entry.size,
        modified: modified_str,
        children: if entry.is_dir {
            Some(Vec::new())
        } else {
            None
        },
        parent_path: std::path::Path::new(&entry.path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string(),
    }
}

/// Convert DTO to domain workspace (if needed)
#[allow(dead_code)]
pub fn dto_to_domain_workspace(
    dto: &crate::commands::workspace::OpenWorkspaceResponse,
) -> zaroxi_domain_workspace::workspace::Workspace {
    use uuid::Uuid;
    use chrono::Utc;
    
    zaroxi_domain_workspace::workspace::Workspace {
        id: Uuid::parse_str(&dto.workspace_id)
            .unwrap_or_else(|_| Uuid::new_v4()),
        root_path: dto.root_path.clone(),
        name: std::path::Path::new(&dto.root_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("workspace")
            .to_string(),
        is_open: true,
        created_at: Utc::now(),
        last_accessed_at: Utc::now(),
    }
}
