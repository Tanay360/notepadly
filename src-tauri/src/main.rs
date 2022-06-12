#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::env;

use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct OpenFilePayload {
  path: String,
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
  
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let s = args.get(args.len()-1);
    let s_str = s.as_deref().unwrap().as_str();
    window.emit_to("main", "open-file", OpenFilePayload {
      path: s_str.into()
    }).expect("Error occurred while emitting event");
  }
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![close_splashscreen])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
