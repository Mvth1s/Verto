use std::fs;
use std::path::Path;

const MAX_FILES: usize = 1000;

#[tauri::command]
pub async fn list_directory(dir_path: String, recursive: bool) -> Result<Vec<String>, String> {
    if !std::path::Path::new(&dir_path).is_absolute() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    fn setup_dir() -> TempDir {
        let dir = tempfile::tempdir().expect("temp dir");
        File::create(dir.path().join("a.txt")).unwrap();
        File::create(dir.path().join("b.png")).unwrap();
        dir
    }

    #[tokio::test]
    async fn test_relative_path_rejected() {
        let result = list_directory("relative/path".to_string(), false).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("absolute"));
    }

    #[tokio::test]
    async fn test_nonexistent_directory_fails() {
        let result = list_directory("/tmp/verto_nonexistent_xyz".to_string(), false).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_flat() {
        let dir = setup_dir();
        let result = list_directory(dir.path().to_str().unwrap().to_string(), false).await;
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
    }

    #[tokio::test]
    async fn test_list_recursive() {
        let dir = setup_dir();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        File::create(sub.join("c.jpg")).unwrap();

        let result = list_directory(dir.path().to_str().unwrap().to_string(), true).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_non_recursive_skips_subdirs() {
        let dir = setup_dir();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        File::create(sub.join("c.jpg")).unwrap();

        let result = list_directory(dir.path().to_str().unwrap().to_string(), false).await;
        assert_eq!(result.unwrap().len(), 2);
    }
}
