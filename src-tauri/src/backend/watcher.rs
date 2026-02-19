use std::{fs::{self}, path::{Path, PathBuf}, sync::mpsc::channel, thread, time::Duration};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::CreateKind};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn start(app: AppHandle) {

    let downloads = initialize_download_directory(app);
    println!("Watching: {:?}", downloads);

    thread::spawn(move || {
        run_watcher(&downloads);
    });
}

fn initialize_download_directory(app: AppHandle) -> PathBuf {
        app.path()
        .download_dir()
        .expect("Failed to resolve downloads directory")
}

fn run_watcher(downloads: &Path) {
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
                        let event_kind = event.kind;
                        if is_file_done_downloading(&path, event_kind) {
                            println!("File download detected: {:?}, {:?}", event.kind, path);
                        }
                    }   
                }
                Err(e) => {
                    println!("Watch error: {:?}", e);
                }
            }
        }
            
}

fn is_file_done_downloading(path: &Path, event_kind: EventKind) -> bool {

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

    let file_size1 = match fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(_) => return false,
    };

    thread::sleep(Duration::from_millis(500));

    let file_size2 = match fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(_) => return false,
    };

    if file_size1 != file_size2 {
        return false;
    }

    if event_kind != EventKind::Create(CreateKind::File) {
        return false;
    }

    if is_hidden(&path) {
        return false;
    }

    true
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}