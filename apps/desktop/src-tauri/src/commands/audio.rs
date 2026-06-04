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
pub async fn convert_audio(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::ActiveConversion>,
    input_path: String,
    output_format: String,
    bitrate: Option<u32>,
    output_path: Option<String>,
    file_id: String,
) -> Result<ConversionResult, String> {
    if !std::path::Path::new(&input_path).is_absolute() {
        return Err("Input path must be absolute".to_string());
    }

    if let Some(ref out) = output_path {
        if !std::path::Path::new(out).is_absolute() {
            return Err("Output path must be absolute".to_string());
        }
    }

    let result = ffmpeg_converter::convert(
        &app,
        &state,
        &input_path,
        &output_format,
        output_path.as_deref(),
        bitrate,
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
