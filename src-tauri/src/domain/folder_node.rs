#[derive(Debug, Clone)]
pub struct FolderNode {
    pub id: String,
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub parent: Option<String>,
    pub children: Vec<String>,
}