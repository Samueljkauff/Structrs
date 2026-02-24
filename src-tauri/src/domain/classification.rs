use std::path::PathBuf;
use crate::backend::file_meta::FileMeta;

#[derive(Debug, Clone)]
pub struct ClassificationResult {
    pub category: String,
    pub confidence: f32,
    pub suggested_path: PathBuf,
    pub reasoning: Option<String>,
}

pub trait Classifier {
    fn classify(meta: &FileMeta) -> Self;
}