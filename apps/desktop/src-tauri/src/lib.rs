mod commands;
mod converters;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::image::convert_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
