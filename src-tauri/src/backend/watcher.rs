use std::{path::{Path, PathBuf}, sync::mpsc::channel, thread};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn start(app: AppHandle) {
    let downloads = app
        .path()
        .download_dir()
        .expect("Failed to resolve downloads directory");

    println!("Watching: {:?}", downloads);

    thread::spawn(move || {
        let (tx, rx) = channel();

        let mut watcher =
            RecommendedWatcher::new(tx, Config::default())
                .expect("Failed to create watcher");

        watcher
            .watch(&downloads, RecursiveMode::Recursive)
            .expect("Failed to watch directory");

        println!("Watcher started");

        for res in rx {
            match res {
                Ok(event) => {
                    for path in event.paths {
                        if is_file_done_downloading(&path) {
                            println!("File is ready: {:?}", path);
                        }
}
                }
                Err(e) => {
                    println!("Watch error: {:?}", e);
                }
            }
        }
    });
}

fn is_file_done_downloading(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }
        if !path.is_file() {
        return false;
    }

    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e,
        None => return false,
    };

    if ext == "crdownload" || ext == "part" || ext == "tmp" {
        return false;
    }

    true
}

