pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub children: Vec<FileNode>,
}