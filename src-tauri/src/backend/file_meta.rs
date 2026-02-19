use std::path::{self, Path};

pub struct file_meta {
    file_name: String,
    extension: Option<String>,
    mime: Option<String>,
    size: u64,
}

impl file_meta {
    pub fn new(path: &Path) ->  std::io::Result<Self> {
        let meta_data = std::fs::metadata(path)?;
        println!("{:?}", meta_data);

        Ok(Self {
            file_name: path.file_name().unwrap_or_default().to_string(),
            extension: Some("".to_string()),
            mime: Some("".to_string()),
            size: meta_data.len(),
        })
    }
}