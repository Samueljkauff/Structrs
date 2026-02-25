use std::path::PathBuf;

use super::file_meta::FileMeta;
use super::classification::{ClassificationResult, Classifier};

pub struct DummyClassifier;

impl Classifier for DummyClassifier {
    fn classify(&self, meta: &FileMeta) -> ClassificationResult {
        let category = match meta.extension.as_deref() {
            Some("png") | Some("jpg") | Some("jpeg") => "Photos",
            Some("pdf") => "Documents",
            _ => "Other",
        }.to_string();

        let suggested_path = match category.as_str() {
            "Photos" => PathBuf::from("Photos"),
            "Documents" => PathBuf::from("Documents"),
            _ => PathBuf::from("Downloads"),
        };

        ClassificationResult {
            category,
            confidence: 1.0,
            suggested_path,
            reasoning: Some("Extension".to_string()),
        }
    }
}

