use crate::converters::ffmpeg as ffmpeg_converter;
use serde::Serialize;

#[derive(Serialize)]
pub struct ConversionResult {
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
    pub saved_bytes: i64,
}

#[tauri::command]
pub async fn convert_video(
    app: tauri::AppHandle,
    input_path: String,
    output_format: String,
    codec: Option<String>,
    resolution_width: Option<u32>,
    resolution_height: Option<u32>,
    output_path: Option<String>,
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
    )
    .await?;

    Ok(ConversionResult {
        saved_bytes: result.input_size as i64 - result.output_size as i64,
        output_path: result.output_path,
        input_size: result.input_size,
        output_size: result.output_size,
    })
}
