// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use notify::{Error as NotifyError, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::io::Error;
use std::path::{Path, PathBuf};
// use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::{thread, time::Duration};
use winrt_notification::Toast;

struct WatchState {
    watcher: Mutex<RecommendedWatcher>,
    watched_paths: Mutex<HashSet<PathBuf>>,
}

impl WatchState {
    fn new() -> Result<Self, NotifyError> {
        let watcher = notify::recommended_watcher(|res: notify::Result<Event>| {
            match res {
                Ok(event) => {
                    // Filter for newly created files
                    if let EventKind::Create(_) = event.kind {
                        for path in event.paths {
                            println!("New file detected: {:?}", path);
                            let new_file_path = Path::new(&path);
                            if is_jpg(new_file_path) {
                                thread::sleep(Duration::from_millis(500));
                                notify(&new_file_path)
                            }
                        }
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        })?;

        Ok(Self {
            watcher: Mutex::new(watcher),
            watched_paths: Mutex::new(HashSet::new()),
        })
    }
}

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

fn add_folder(path: &str, state: tauri::State<WatchState>) -> Result<(), Error> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() || !path_buf.is_dir() {
        return Err(Error::other("Path does not exist or is not a directory"));
    }

    let mut watched_paths = state
        .watched_paths
        .lock()
        .map_err(|_| Error::other("Failed to acquire lock"))?;

    if watched_paths.contains(&path_buf) {
        return Err(Error::other("Folder is already being watched"));
    }

    watched_paths.insert(path_buf);

    let path_buf_to_watch = PathBuf::from(&path);

    let mut watcher = state
        .watcher
        .lock()
        .map_err(|_| Error::other("Failed to acquire watcher lock"))?;

    watcher
        .watch(&path_buf_to_watch, RecursiveMode::Recursive)
        .map_err(|e| Error::other(format!("Failed to watch folder: {e}")))?;

    println!("Watched folders: {:?}", *watched_paths);

    Ok(())
}

/* fn watch_folder(folder_path: &str) -> NotifyResult<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx)?;

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
                        if is_jpg(new_file_path) {
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
 */
#[tauri::command]
fn start_watching_folder(path: &str, state: tauri::State<WatchState>) -> Result<(), String> {
    // let path_string = path.to_string();

    // thread::spawn(move || watch_folder(&path_string));

    add_folder(&path, state).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn stop_watching_folder(path: &str, state: tauri::State<WatchState>) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);

    let mut watched_paths = state
        .watched_paths
        .lock()
        .map_err(|_| "Failed to acquire lock".to_string())?;

    if !watched_paths.contains(&path_buf) {
        return Err("Folder is not being watched".to_string());
    }

    let mut watcher = state
        .watcher
        .lock()
        .map_err(|_| "Failed to acquire watcher lock".to_string())?;

    watcher
        .unwatch(&path_buf)
        .map_err(|e| format!("Failed to stop watching folder: {e}"))?;

    watched_paths.remove(&path_buf);

    println!("Stopped watching: {:?}", path_buf);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .manage(WatchState::new().expect("failed to initialize WatchState"))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_watching_folder,
            stop_watching_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
