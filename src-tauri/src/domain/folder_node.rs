use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderNode {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub parent: Option<String>,
    pub children: Vec<String>,
}