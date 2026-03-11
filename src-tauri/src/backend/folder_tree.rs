use walkdir::WalkDir;

#[tauri::command]
fn load_children(root: String) -> Vec<FileNode> {
    WalkDir::new(root).into_iter().filter_map(|e| e.ok());
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}