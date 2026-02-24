use std::path::PathBuf;

use crate::domain::file_meta::FileMeta;

#[derive(Debug, Clone)]
pub struct ClassificationResult {
    pub category: String,
    pub confidence: f32,
    pub suggested_path: PathBuf,
    pub reasoning: Option<String>,
}

pub trait Classifier {
    fn classify(&self, meta: &FileMeta) -> ClassificationResult;
}