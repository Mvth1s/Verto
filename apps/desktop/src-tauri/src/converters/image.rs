use image::ImageFormat;
use std::path::Path;

#[derive(Debug)]
pub struct ConversionResult {
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
}

pub fn convert(
    input_path: &str,
    output_format: &str,
    quality: Option<u8>,
    output_path: Option<&str>,
    resize: Option<(u32, u32)>,
) -> Result<ConversionResult, String> {
    let input = Path::new(input_path);

    if !input.exists() {
        return Err(format!("File not found: {}", input_path));
    }

    let format = parse_format(output_format)?;

    let out_path = match output_path {
        Some(p) => p.to_string(),
        None => {
            let stem = input
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or("Invalid file name")?;
            let parent = input.parent().unwrap_or(Path::new("."));
            parent
                .join(format!("{}.{}", stem, format_to_extension(format)))
                .to_string_lossy()
                .into_owned()
        }
    };

    let input_size = std::fs::metadata(input_path)
        .map_err(|e| e.to_string())?
        .len();

    let mut img = image::open(input).map_err(|e| format!("Failed to open image: {}", e))?;

    if let Some((w, h)) = resize {
        img = img.resize(w, h, image::imageops::FilterType::Lanczos3);
    }

    match format {
        ImageFormat::Jpeg => {
            let q = quality.unwrap_or(85);
            let mut file = std::fs::File::create(&out_path).map_err(|e| e.to_string())?;
            img.write_with_encoder(image::codecs::jpeg::JpegEncoder::new_with_quality(
                &mut file, q,
            ))
            .map_err(|e| e.to_string())?;
        }
        ImageFormat::Ico => {
            // ICO encoder requires RGBA and supports at most 256x256 pixels.
            let img = if img.width() > 256 || img.height() > 256 {
                img.thumbnail(256, 256)
            } else {
                img
            };
            image::DynamicImage::ImageRgba8(img.into_rgba8())
                .save_with_format(&out_path, ImageFormat::Ico)
                .map_err(|e| e.to_string())?;
        }
        _ => {
            img.save_with_format(&out_path, format)
                .map_err(|e| e.to_string())?;
        }
    }

    let output_size = std::fs::metadata(&out_path)
        .map_err(|e| e.to_string())?
        .len();

    Ok(ConversionResult {
        output_path: out_path,
        input_size,
        output_size,
    })
}

fn parse_format(s: &str) -> Result<ImageFormat, String> {
    match s.to_lowercase().as_str() {
        "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
        "png" => Ok(ImageFormat::Png),
        "webp" => Ok(ImageFormat::WebP),
        "bmp" => Ok(ImageFormat::Bmp),
        "tiff" | "tif" => Ok(ImageFormat::Tiff),
        "gif" => Ok(ImageFormat::Gif),
        "ico" => Ok(ImageFormat::Ico),
        _ => Err(format!("Unsupported format: {}", s)),
    }
}

fn format_to_extension(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Png => "png",
        ImageFormat::WebP => "webp",
        ImageFormat::Bmp => "bmp",
        ImageFormat::Tiff => "tiff",
        ImageFormat::Gif => "gif",
        ImageFormat::Ico => "ico",
        _ => "bin",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture(name: &str) -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn test_parse_format_jpeg() {
        assert!(matches!(parse_format("jpeg"), Ok(ImageFormat::Jpeg)));
        assert!(matches!(parse_format("jpg"), Ok(ImageFormat::Jpeg)));
        assert!(matches!(parse_format("JPEG"), Ok(ImageFormat::Jpeg)));
    }

    #[test]
    fn test_parse_format_ico() {
        assert!(matches!(parse_format("ico"), Ok(ImageFormat::Ico)));
        assert!(matches!(parse_format("ICO"), Ok(ImageFormat::Ico)));
    }

    #[test]
    fn test_format_to_extension_ico() {
        assert_eq!(format_to_extension(ImageFormat::Ico), "ico");
    }

    #[test]
    fn test_parse_format_unsupported() {
        assert!(parse_format("avif").is_err());
        assert!(parse_format("heic").is_err());
    }

    #[test]
    fn test_png_to_ico() {
        let input = fixture("sample.png");
        let output = std::env::temp_dir()
            .join("verto_test_png_to_ico.ico")
            .to_string_lossy()
            .into_owned();

        if !std::path::Path::new(&input).exists() {
            return;
        }

        let result = convert(&input, "ico", None, Some(&output), None);
        assert!(result.is_ok(), "PNG to ICO failed: {:?}", result.err());
        assert!(std::path::Path::new(&output).exists());
        let bytes = std::fs::read(&output).unwrap();
        assert_eq!(&bytes[0..4], &[0, 0, 1, 0], "ICO magic bytes should be 00 00 01 00");
        let _ = std::fs::remove_file(&output);
    }

    #[test]
    fn test_jpeg_to_ico() {
        let input = fixture("sample.jpg");
        let output = std::env::temp_dir()
            .join("verto_test_jpeg_to_ico.ico")
            .to_string_lossy()
            .into_owned();

        if !std::path::Path::new(&input).exists() {
            return;
        }

        let result = convert(&input, "ico", None, Some(&output), None);
        assert!(result.is_ok(), "JPEG to ICO failed: {:?}", result.err());
        assert!(std::path::Path::new(&output).exists());
        let bytes = std::fs::read(&output).unwrap();
        assert_eq!(&bytes[0..4], &[0, 0, 1, 0], "ICO magic bytes expected");
        let _ = std::fs::remove_file(&output);
    }

    #[test]
    fn test_ico_to_png() {
        let input = fixture("sample.ico");
        let output = std::env::temp_dir()
            .join("verto_test_ico_to_png.png")
            .to_string_lossy()
            .into_owned();

        if !std::path::Path::new(&input).exists() {
            return;
        }

        let result = convert(&input, "png", None, Some(&output), None);
        assert!(result.is_ok(), "ICO to PNG failed: {:?}", result.err());
        assert!(std::path::Path::new(&output).exists());
        let bytes = std::fs::read(&output).unwrap();
        assert_eq!(&bytes[0..4], &[137, 80, 78, 71], "Output should be valid PNG");
        let _ = std::fs::remove_file(&output);
    }

    #[test]
    fn test_format_to_extension() {
        assert_eq!(format_to_extension(ImageFormat::Jpeg), "jpg");
        assert_eq!(format_to_extension(ImageFormat::Png), "png");
        assert_eq!(format_to_extension(ImageFormat::WebP), "webp");
    }

    #[test]
    fn test_convert_nonexistent_file() {
        let result = convert("/nonexistent/path/file.png", "webp", None, None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File not found"));
    }

    #[test]
    fn test_jpeg_to_png() {
        let input = fixture("sample.jpg");
        let output = std::env::temp_dir()
            .join("verto_test_jpeg_to_png.png")
            .to_string_lossy()
            .into_owned();

        if !std::path::Path::new(&input).exists() {
            return; // Skip if fixture missing
        }

        let result = convert(&input, "png", None, Some(&output), None);
        assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
        assert!(std::path::Path::new(&output).exists());
        let _ = std::fs::remove_file(&output);
    }

    #[test]
    fn test_png_to_webp() {
        let input = fixture("sample.png");
        let output = std::env::temp_dir()
            .join("verto_test_png_to_webp.webp")
            .to_string_lossy()
            .into_owned();

        if !std::path::Path::new(&input).exists() {
            return;
        }

        let result = convert(&input, "webp", None, Some(&output), None);
        assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
        assert!(std::path::Path::new(&output).exists());
        let _ = std::fs::remove_file(&output);
    }

    #[test]
    fn test_jpeg_quality() {
        let input = fixture("sample.jpg");
        let output_high = std::env::temp_dir()
            .join("verto_test_quality_high.jpg")
            .to_string_lossy()
            .into_owned();
        let output_low = std::env::temp_dir()
            .join("verto_test_quality_low.jpg")
            .to_string_lossy()
            .into_owned();

        if !std::path::Path::new(&input).exists() {
            return;
        }

        let high = convert(&input, "jpeg", Some(95), Some(&output_high), None);
        let low = convert(&input, "jpeg", Some(10), Some(&output_low), None);

        assert!(high.is_ok());
        assert!(low.is_ok());

        let high_size = high.unwrap().output_size;
        let low_size = low.unwrap().output_size;
        assert!(
            high_size > low_size,
            "High quality should produce larger file"
        );

        let _ = std::fs::remove_file(&output_high);
        let _ = std::fs::remove_file(&output_low);
    }
}
