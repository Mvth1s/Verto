use crate::converters::ffmpeg as ffmpeg_converter;
use base64::Engine;
use serde::Serialize;
use tauri_plugin_shell::ShellExt;

#[derive(Serialize)]
pub struct ConversionResult {
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
    pub saved_bytes: i64,
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn convert_video(
    app: tauri::AppHandle,
    input_path: String,
    output_format: String,
    codec: Option<String>,
    resolution_width: Option<u32>,
    resolution_height: Option<u32>,
    output_path: Option<String>,
    file_id: String,
) -> Result<ConversionResult, String> {
    if !input_path.starts_with('/') {
        return Err("Input path must be absolute".to_string());
    }

    if let Some(ref out) = output_path {
        if !out.starts_with('/') {
            return Err("Output path must be absolute".to_string());
        }
    }

    let out = output_path.unwrap_or_else(|| {
        std::path::Path::new(&input_path)
            .with_extension(&output_format)
            .to_string_lossy()
            .into_owned()
    });

    let result = ffmpeg_converter::convert_video(
        &app,
        &input_path,
        &output_format,
        &out,
        codec.as_deref(),
        resolution_width,
        resolution_height,
        &file_id,
    )
    .await?;

    Ok(ConversionResult {
        saved_bytes: result.input_size as i64 - result.output_size as i64,
        output_path: result.output_path,
        input_size: result.input_size,
        output_size: result.output_size,
    })
}

#[tauri::command]
pub async fn get_video_thumbnail(
    app: tauri::AppHandle,
    input_path: String,
) -> Result<String, String> {
    if !input_path.starts_with('/') {
        return Err("Input path must be absolute".to_string());
    }
    if !std::path::Path::new(&input_path).exists() {
        return Err(format!("File not found: {}", input_path));
    }

    let tmp = std::env::temp_dir().join(format!(
        "verto_thumb_{}.jpg",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos()
    ));

    let (mut rx, _child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| e.to_string())?
        .args([
            "-y",
            "-ss",
            "0.1",
            "-i",
            &input_path,
            "-vframes",
            "1",
            "-vf",
            "scale=64:-2",
            "-q:v",
            "5",
            tmp.to_str().ok_or("invalid temp path")?,
        ])
        .spawn()
        .map_err(|e| e.to_string())?;

    while let Some(event) = rx.recv().await {
        if let tauri_plugin_shell::process::CommandEvent::Terminated(_) = event {
            break;
        }
    }

    let bytes = std::fs::read(&tmp).map_err(|e| format!("Failed to read thumbnail: {}", e))?;
    let _ = std::fs::remove_file(&tmp);

    Ok(format!(
        "data:image/jpeg;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(&bytes)
    ))
}
