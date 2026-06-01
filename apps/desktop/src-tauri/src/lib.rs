mod commands;
mod converters;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            commands::image::convert_image,
            commands::document::convert_document,
            commands::audio::convert_audio,
            commands::fs::list_directory,
            commands::updater::check_for_updates,
            commands::updater::install_update,
            commands::video::convert_video,
            commands::video::get_video_thumbnail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
