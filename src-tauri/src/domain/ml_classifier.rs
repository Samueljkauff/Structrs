use std::path::PathBuf;

use super::file_meta::FileMeta;
use super::classification::{ClassificationResult, Classifier};

pub struct MLClassifier;

impl Classifier for MLClassifier {
    fn classify(&self, meta: &FileMeta) -> ClassificationResult {
        return ClassificationResult { category: "".to_string(), confidence: 1.0, suggested_path: PathBuf::from(""), reasoning: Some("".to_string()) };
    }
}

