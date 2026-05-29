use std::fs;
use std::path::Path;

const MAX_FILES: usize = 1000;

#[tauri::command]
pub async fn list_directory(dir_path: String, recursive: bool) -> Result<Vec<String>, String> {
    if !dir_path.starts_with('/') {
        return Err("Directory path must be absolute".to_string());
    }

    let mut files: Vec<String> = Vec::new();
    collect_files(Path::new(&dir_path), recursive, &mut files)?;
    Ok(files)
}

fn collect_files(dir: &Path, recursive: bool, files: &mut Vec<String>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        if files.len() >= MAX_FILES {
            break;
        }

        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(p) = path.to_str() {
                files.push(p.to_string());
            }
        } else if path.is_dir() && recursive {
            collect_files(&path, recursive, files)?;
        }
    }

    Ok(())
}
