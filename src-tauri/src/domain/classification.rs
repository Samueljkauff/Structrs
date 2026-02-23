use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ClassificationResult {
    pub category: String,
    pub confidence: f32,
    pub suggested_path: PathBuf,
    pub reasoning: Option<String>,
}