use crate::converters::ffmpeg as ffmpeg_converter;
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

/// Formats that require FFmpeg for encoding (output).
const FFMPEG_OUTPUT_FORMATS: &[&str] = &["avif"];
/// Formats that require FFmpeg for decoding (input only, no encode support via image crate).
const FFMPEG_INPUT_FORMATS: &[&str] = &[
    "heic", "heif", // Apple formats
    "psd",  // Photoshop (FFmpeg extracts flattened render)
    "dds",  // DirectDraw Surface (textures)
    "exr",  // OpenEXR HDR
    "qoi",  // Quite OK Image - FFmpeg >= 5.1 required
];

#[tauri::command]
pub async fn convert_image(
    app: tauri::AppHandle,
    input_path: String,
    output_format: String,
    quality: Option<u8>,
    resize_width: Option<u32>,
    resize_height: Option<u32>,
    output_path: Option<String>,
) -> Result<ConversionResult, String> {
    let input = Path::new(&input_path);
    if !input.is_absolute() {
        return Err("Input path must be absolute".to_string());
    }

    let out_path = match output_path {
        Some(ref p) => {
            if !Path::new(p).is_absolute() {
                return Err("Output path must be absolute".to_string());
            }
            p.clone()
        }
        None => input
            .with_extension(&output_format)
            .to_string_lossy()
            .into_owned(),
    };

    let input_ext = input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let use_ffmpeg = FFMPEG_OUTPUT_FORMATS.contains(&output_format.as_str())
        || FFMPEG_OUTPUT_FORMATS.contains(&input_ext.as_str())
        || FFMPEG_INPUT_FORMATS.contains(&input_ext.as_str());

    if use_ffmpeg {
        let resize = match (resize_width, resize_height) {
            (None, None) => None,
            pair => Some(pair),
        };
        let result =
            ffmpeg_converter::convert_image(&app, &input_path, &output_format, &out_path, resize)
                .await?;

        return Ok(ConversionResult {
            saved_bytes: result.input_size as i64 - result.output_size as i64,
            output_path: result.output_path,
            input_size: result.input_size,
            output_size: result.output_size,
        });
    }

    let resize = match (resize_width, resize_height) {
        (Some(w), Some(h)) => Some((w, h)),
        (Some(w), None) => Some((w, u32::MAX)),
        (None, Some(h)) => Some((u32::MAX, h)),
        (None, None) => None,
    };
    let out_path_clone = out_path.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        image_converter::convert(
            &input_path,
            &output_format,
            quality,
            Some(&out_path_clone),
            resize,
        )
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(ConversionResult {
        saved_bytes: result.input_size as i64 - result.output_size as i64,
        output_path: result.output_path,
        input_size: result.input_size,
        output_size: result.output_size,
    })
}
