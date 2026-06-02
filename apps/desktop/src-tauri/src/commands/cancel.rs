use crate::ActiveConversion;

#[tauri::command]
pub fn cancel_conversion(state: tauri::State<'_, ActiveConversion>) -> Result<(), String> {
    if let Some(child) = state.0.lock().map_err(|e| e.to_string())?.take() {
        child.kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}
