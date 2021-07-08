#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command(with_window)]
async fn close_splashscreen<M: tauri::Params>(window: tauri::Window<M>) {
  if let Ok(splashscreen) = window.get_webview("splash") {
    splashscreen.close().unwrap();
  }
  window.get_webview("main").unwrap().show().unwrap();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![close_splashscreen])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
