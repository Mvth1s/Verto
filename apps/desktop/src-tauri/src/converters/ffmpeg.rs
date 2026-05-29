use serde::Serialize;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Serialize)]
pub struct ConversionResult {
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
}

const ALLOWED_FORMATS: &[&str] = &["mp3", "flac", "ogg", "wav", "aac", "opus", "m4a"];

pub async fn convert(
    app: &tauri::AppHandle,
    input_path: &str,
    output_format: &str,
    output_path: Option<&str>,
    bitrate: Option<u32>,
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

    let mut args = vec!["-y".to_string(), "-i".to_string(), input_path.to_string()];

    if let Some(br) = bitrate {
        args.push("-b:a".to_string());
        args.push(format!("{}k", br));
    }

    args.push(out_path.clone());

    let (mut rx, _child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| e.to_string())?
        .args(&args)
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
        return Err(format!("ffmpeg failed: {}", stderr_buf.trim()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn ffmpeg_bin() -> Option<PathBuf> {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("binaries");
        [
            "ffmpeg-x86_64-unknown-linux-gnu",
            "ffmpeg-aarch64-unknown-linux-gnu",
            "ffmpeg-x86_64-apple-darwin",
            "ffmpeg-aarch64-apple-darwin",
            "ffmpeg-x86_64-pc-windows-msvc.exe",
        ]
        .iter()
        .map(|name| base.join(name))
        .find(|p| p.exists() && p.metadata().map(|m| m.len() > 0).unwrap_or(false))
    }

    fn create_minimal_wav(path: &PathBuf) {
        const SAMPLE_RATE: u32 = 44100;
        const NUM_SAMPLES: u32 = 4410; // 0.1 second
        const CHANNELS: u16 = 1;
        const BITS: u16 = 16;
        let byte_rate = SAMPLE_RATE * CHANNELS as u32 * (BITS as u32 / 8);
        let block_align = CHANNELS * (BITS / 8);
        let data_size = NUM_SAMPLES * CHANNELS as u32 * (BITS as u32 / 8);
        let riff_size = 36 + data_size;

        let mut buf = Vec::with_capacity(44 + data_size as usize);
        buf.extend_from_slice(b"RIFF");
        buf.extend_from_slice(&riff_size.to_le_bytes());
        buf.extend_from_slice(b"WAVE");
        buf.extend_from_slice(b"fmt ");
        buf.extend_from_slice(&16u32.to_le_bytes());
        buf.extend_from_slice(&1u16.to_le_bytes()); // PCM
        buf.extend_from_slice(&CHANNELS.to_le_bytes());
        buf.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
        buf.extend_from_slice(&byte_rate.to_le_bytes());
        buf.extend_from_slice(&block_align.to_le_bytes());
        buf.extend_from_slice(&BITS.to_le_bytes());
        buf.extend_from_slice(b"data");
        buf.extend_from_slice(&data_size.to_le_bytes());
        buf.extend(std::iter::repeat(0u8).take(data_size as usize));
        std::fs::write(path, &buf).unwrap();
    }

    // ── Format validation ─────────────────────────────────────────────────────

    #[test]
    fn test_allowed_formats_accepted() {
        for fmt in &["mp3", "flac", "ogg", "wav", "aac", "opus", "m4a"] {
            assert!(ALLOWED_FORMATS.contains(fmt), "{} should be allowed", fmt);
        }
    }

    #[test]
    fn test_disallowed_formats_rejected() {
        for fmt in &["jpg", "png", "docx", "html", "txt", ""] {
            assert!(
                !ALLOWED_FORMATS.contains(fmt),
                "{} should not be allowed",
                fmt
            );
        }
    }

    // ── Output path computation ───────────────────────────────────────────────

    #[test]
    fn test_output_path_replaces_extension() {
        let p = PathBuf::from("/tmp/track.wav").with_extension("mp3");
        assert_eq!(p.to_str().unwrap(), "/tmp/track.mp3");
    }

    #[test]
    fn test_output_path_flac_to_ogg() {
        let p = PathBuf::from("/home/user/music.flac").with_extension("ogg");
        assert_eq!(p.to_str().unwrap(), "/home/user/music.ogg");
    }

    // ── Integration: real FFmpeg sidecar ──────────────────────────────────────

    #[test]
    fn test_wav_to_mp3() {
        let ffmpeg = match ffmpeg_bin() {
            Some(p) => p,
            None => return,
        };
        let wav = std::env::temp_dir().join("verto_test_ffmpeg_input.wav");
        create_minimal_wav(&wav);

        let output = std::env::temp_dir()
            .join("verto_test_ffmpeg_wav_to_mp3.mp3")
            .to_string_lossy()
            .into_owned();

        let status = std::process::Command::new(&ffmpeg)
            .args(["-y", "-i", wav.to_str().unwrap(), &output])
            .status()
            .expect("failed to run ffmpeg");

        assert!(status.success(), "ffmpeg wav→mp3 failed");
        assert!(PathBuf::from(&output).exists());
        let bytes = std::fs::read(&output).unwrap();
        assert!(!bytes.is_empty(), "mp3 output should not be empty");
        let _ = std::fs::remove_file(&output);
        let _ = std::fs::remove_file(&wav);
    }

    #[test]
    fn test_wav_to_flac() {
        let ffmpeg = match ffmpeg_bin() {
            Some(p) => p,
            None => return,
        };
        let wav = std::env::temp_dir().join("verto_test_ffmpeg_input2.wav");
        create_minimal_wav(&wav);

        let output = std::env::temp_dir()
            .join("verto_test_ffmpeg_wav_to_flac.flac")
            .to_string_lossy()
            .into_owned();

        let status = std::process::Command::new(&ffmpeg)
            .args(["-y", "-i", wav.to_str().unwrap(), &output])
            .status()
            .expect("failed to run ffmpeg");

        assert!(status.success(), "ffmpeg wav→flac failed");
        assert!(PathBuf::from(&output).exists());
        let bytes = std::fs::read(&output).unwrap();
        assert_eq!(&bytes[..4], b"fLaC", "output should be a valid FLAC file");
        let _ = std::fs::remove_file(&output);
        let _ = std::fs::remove_file(&wav);
    }

    #[test]
    fn test_nonexistent_input_fails() {
        let ffmpeg = match ffmpeg_bin() {
            Some(p) => p,
            None => return,
        };
        let output = std::env::temp_dir()
            .join("verto_test_ffmpeg_nonexistent.mp3")
            .to_string_lossy()
            .into_owned();

        let status = std::process::Command::new(&ffmpeg)
            .args(["-y", "-i", "/nonexistent/path/audio.wav", &output])
            .status()
            .expect("failed to run ffmpeg");

        assert!(!status.success(), "ffmpeg should fail on nonexistent input");
    }
}
