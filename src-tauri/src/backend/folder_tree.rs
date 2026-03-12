use walkdir::WalkDir;
use crate::domain::folder_node::FolderNode;


#[tauri::command]
pub async fn load_children(root: String) -> Vec<FolderNode> {
    WalkDir::new(root).into_iter().filter_map(|e| e.ok());
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}