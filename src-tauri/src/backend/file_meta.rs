use std::path::Path;
use mime_guess::from_ext;

pub struct file_meta {
    file_name: String,
    extension: Option<String>,
    mime: Option<String>,
    size: u64,
}

impl file_meta {
    pub fn new(path: &Path) ->  std::io::Result<Self> {
        let meta_data = std::fs::metadata(path)?;
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let extension = path.extension().map(|ext| ext.to_string_lossy().to_string());
        let mime = extension.as_ref().map(|ext| Self::infer_mime(ext));
        let size = meta_data.len();

        println!("{:?}", meta_data);

        Ok(Self {
            file_name,
            extension,
            mime,
            size,
        })
    }

    fn infer_mime(ext: &str) -> String {
    from_ext(ext)
        .first_or_octet_stream()
        .essence_str()
        .to_string()
}
}