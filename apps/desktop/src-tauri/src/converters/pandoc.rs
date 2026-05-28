use serde::Serialize;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Serialize)]
pub struct ConversionResult {
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
}

const ALLOWED_FORMATS: &[&str] = &["html", "pdf", "docx", "md", "rst", "odt", "epub"];

pub async fn convert(
    app: &tauri::AppHandle,
    input_path: &str,
    output_format: &str,
    output_path: Option<&str>,
) -> Result<ConversionResult, String> {
    if !std::path::Path::new(input_path).exists() {
        return Err(format!("Input file not found: {}", input_path));
    }

    if !ALLOWED_FORMATS.contains(&output_format) {
        return Err(format!(
            "Unsupported output format: {}. Allowed: {}",
            output_format,
            ALLOWED_FORMATS.join(", ")
        ));
    }

    let out_path = match output_path {
        Some(p) => p.to_string(),
        None => std::path::Path::new(input_path)
            .with_extension(output_format)
            .to_str()
            .ok_or("Failed to compute output path")?
            .to_string(),
    };

    if !input_path.starts_with('/') {
        return Err("Input path must be absolute".to_string());
    }
    if !out_path.starts_with('/') {
        return Err("Output path must be absolute".to_string());
    }

    let input_size = std::fs::metadata(input_path)
        .map_err(|e| format!("Failed to read input metadata: {}", e))?
        .len();

    let (mut rx, _child) = app
        .shell()
        .sidecar("pandoc")
        .map_err(|e| e.to_string())?
        .args([input_path, "-o", &out_path])
        .spawn()
        .map_err(|e| e.to_string())?;

    let mut stderr_buf = String::new();
    let mut exit_code: Option<i32> = None;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stderr(line) => {
                stderr_buf.push_str(&String::from_utf8_lossy(&line));
            }
            CommandEvent::Terminated(payload) => {
                exit_code = payload.code;
                break;
            }
            _ => {}
        }
    }

    if exit_code != Some(0) {
        return Err(format!("pandoc failed: {}", stderr_buf.trim()));
    }

    let output_size = std::fs::metadata(&out_path)
        .map_err(|e| format!("Failed to read output metadata: {}", e))?
        .len();

    Ok(ConversionResult {
        output_path: out_path,
        input_size,
        output_size,
    })
}
