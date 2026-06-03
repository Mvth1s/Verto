use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn open_output_folder(app: tauri::AppHandle, path: String) -> Result<(), String> {
    app.opener()
        .open_path(&path, None::<&str>)
        .map_err(|e| e.to_string())
}
