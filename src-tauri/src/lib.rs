// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::{thread, time::Duration};
use winrt_notification::Toast;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn is_jpg(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => {
            let ext = ext.to_lowercase();
            ext == "jpg" || ext == "jpeg"
        }
        None => false,
    }
}

fn notify(image: &Path) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Warning")
        .text1("Movement detected")
        .image(image, "Preview image")
        .show()
        .unwrap();
}

fn watch_folder(folder_path: &str) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, notify::Config::default())?;

    let path = Path::new(folder_path);

    watcher.watch(path, RecursiveMode::NonRecursive)?;
    println!("Watching {:?}", path);

    for res in rx {
        match res {
            Ok(event) => {
                // Filter for newly created files
                if let EventKind::Create(_) = event.kind {
                    for path in event.paths {
                        println!("New file detected: {:?}", path);
                        let new_file_path = Path::new(&path);
                        if is_jpg(Path::new(&new_file_path)) {
                            thread::sleep(Duration::from_millis(500));
                            notify(&new_file_path)
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

#[tauri::command]
fn start_watching_folder(path: &str) {
    let path = path.to_string();

    thread::spawn(move || watch_folder(&path));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_watching_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
