use crate::converters::image as image_converter;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct ConversionResult {
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
    pub saved_bytes: i64,
}

#[tauri::command]
pub async fn convert_image(
    input_path: String,
    output_format: String,
    quality: Option<u8>,
    output_path: Option<String>,
) -> Result<ConversionResult, String> {
    let input = Path::new(&input_path);
    if !input.is_absolute() {
        return Err("Input path must be absolute".to_string());
    }

    if let Some(ref out) = output_path {
        let out_p = Path::new(out);
        if !out_p.is_absolute() {
            return Err("Output path must be absolute".to_string());
        }
    }

    // Offload blocking I/O to a dedicated thread
    tauri::async_runtime::spawn_blocking(move || {
        let result =
            image_converter::convert(&input_path, &output_format, quality, output_path.as_deref())?;

        Ok(ConversionResult {
            saved_bytes: result.input_size as i64 - result.output_size as i64,
            output_path: result.output_path,
            input_size: result.input_size,
            output_size: result.output_size,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}
