use serde::Serialize;
use tauri::Emitter;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[derive(Clone, Serialize)]
struct ProgressPayload {
    id: String,
    percent: f32,
}

/// Extracts the most meaningful error line from FFmpeg stderr.
/// Filters out version headers, indented config output, stream info, and stats.
fn extract_ffmpeg_error(stderr: &str) -> String {
    let is_noise = |line: &str| -> bool {
        let t = line.trim();
        if t.is_empty() || t.eq_ignore_ascii_case("conversion failed!") {
            return true;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            return true;
        }
        if t.starts_with("ffmpeg version") || t.starts_with("built with") {
            return true;
        }
        if t.starts_with("Input #")
            || t.starts_with("Output #")
            || t.starts_with("Stream #")
            || t.starts_with("Stream mapping")
            || t.starts_with("Duration:")
            || t.starts_with("Metadata:")
            || t.starts_with("Press [q]")
        {
            return true;
        }
        if t.starts_with('[') {
            return true;
        }
        if t.starts_with("frame=")
            || t.starts_with("Lsize=")
            || t.starts_with("video:")
            || t.starts_with("audio:")
        {
            return true;
        }
        false
    };

    stderr
        .lines()
        .rfind(|l| !is_noise(l))
        .map(|l| {
            let t = l.trim();
            if t.len() > 200 {
                format!("{}…", &t[..200])
            } else {
                t.to_string()
            }
        })
        .unwrap_or_else(|| "FFmpeg conversion failed".to_string())
}

/// Extracts total duration in microseconds from an FFmpeg stderr line.
/// Looks for: `Duration: HH:MM:SS.cc`
fn parse_duration_us(text: &str) -> Option<u64> {
    let pos = text.find("Duration: ")?;
    let s = text[pos + 10..].split(',').next()?.trim();
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let h: u64 = parts[0].trim().parse().ok()?;
    let m: u64 = parts[1].parse().ok()?;
    let sec_parts: Vec<&str> = parts[2].split('.').collect();
    let sec: u64 = sec_parts[0].parse().ok()?;
    let frac_us: u64 = if sec_parts.len() > 1 {
        let frac_str = sec_parts[1];
        let frac: u64 = frac_str.parse().ok()?;
        match frac_str.len() {
            1 => frac * 100_000,
            2 => frac * 10_000,
            3 => frac * 1_000,
            4 => frac * 100,
            5 => frac * 10,
            _ => frac,
        }
    } else {
        0
    };
    Some((h * 3600 + m * 60 + sec) * 1_000_000 + frac_us)
}

/// Parses `out_time_us=<value>` from an FFmpeg `-progress pipe:1` stdout line.
fn parse_out_time_us(text: &str) -> Option<u64> {
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("out_time_us=") {
            let val: i64 = rest.trim().parse().ok()?;
            if val < 0 {
                return None;
            }
            return Some(val as u64);
        }
    }
    None
}

#[derive(Debug, Serialize)]
pub struct ConversionResult {
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
}

const ALLOWED_FORMATS: &[&str] = &[
    // Lossy modern
    "mp3", "aac", "m4a", "opus", "ogg", "spx",
    // Lossy legacy (may fail if FFmpeg not built with these codecs)
    "wma", "amr", "gsm", "mp2", "ra",
    // Lossless
    "flac", "wav", "aiff", "aif", "wv", "ape", "tta", "caf", "au",
    // Broadcast
    "ac3", "eac3", "dts", "mka",
];

/// Explicit audio codec for pure audio output.
fn audio_codec_for_format(output_format: &str) -> Result<&'static str, String> {
    match output_format {
        // Lossless
        "wav"          => Ok("pcm_s16le"),
        "flac"         => Ok("flac"),
        "aiff" | "aif" => Ok("pcm_s16be"),
        "wv"           => Ok("wavpack"),
        "ape"          => Ok("ape"),
        "tta"          => Ok("tta"),
        "caf" | "au"   => Ok("pcm_s16be"),
        // Lossy modern
        "mp3"          => Ok("libmp3lame"),
        "aac" | "m4a"  => Ok("aac"),
        "opus"         => Ok("libopus"),
        "ogg"          => Ok("libvorbis"),
        "spx"          => Ok("libspeex"),
        // Lossy legacy (may fail if FFmpeg not built with these codecs)
        "wma"          => Ok("wmav2"),
        "amr"          => Ok("libopencore_amrnb"),
        "gsm"          => Ok("libgsm"),
        "ra"           => Ok("real_144"),
        // Broadcast
        "mka"          => Ok("flac"),
        "ac3"          => Ok("ac3"),
        "eac3"         => Ok("eac3"),
        "dts"          => Ok("dca"),
        "mp2"          => Ok("mp2"),
        other => Err(format!("Unsupported audio format: {}", other)),
    }
}

pub async fn convert(
    app: &tauri::AppHandle,
    active: &crate::ActiveConversion,
    input_path: &str,
    output_format: &str,
    output_path: Option<&str>,
    bitrate: Option<u32>,
    file_id: &str,
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

    if !std::path::Path::new(input_path).is_absolute() {
        return Err("Input path must be absolute".to_string());
    }
    if !std::path::Path::new(&out_path).is_absolute() {
        return Err("Output path must be absolute".to_string());
    }

    let input_size = std::fs::metadata(input_path)
        .map_err(|e| format!("Failed to read input metadata: {}", e))?
        .len();

    let mut args = vec![
        "-y".to_string(),
        "-i".to_string(),
        input_path.to_string(),
        "-progress".to_string(),
        "pipe:1".to_string(),
        "-nostats".to_string(),
    ];

    let codec = audio_codec_for_format(output_format)?;
    args.push("-c:a".to_string());
    args.push(codec.to_string());

    if let Some(br) = bitrate {
        args.push("-b:a".to_string());
        args.push(format!("{}k", br));
    }

    args.push(out_path.clone());

    let (mut rx, child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| e.to_string())?
        .args(&args)
        .spawn()
        .map_err(|e| e.to_string())?;

    *active.0.lock().map_err(|e| e.to_string())? = Some(child);

    let mut stderr_buf = String::new();
    let mut exit_code: Option<i32> = None;
    let mut duration_us: Option<u64> = None;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line) => {
                let text = String::from_utf8_lossy(&line);
                if let (Some(dur), Some(pos)) = (duration_us, parse_out_time_us(&text)) {
                    if dur > 0 {
                        let percent = ((pos as f64 / dur as f64) * 100.0).min(99.0) as f32;
                        let _ = app.emit(
                            "conversion-progress",
                            ProgressPayload {
                                id: file_id.to_string(),
                                percent,
                            },
                        );
                    }
                }
            }
            CommandEvent::Stderr(line) => {
                let text = String::from_utf8_lossy(&line);
                if duration_us.is_none() {
                    duration_us = parse_duration_us(&text);
                }
                stderr_buf.push_str(&text);
            }
            CommandEvent::Terminated(payload) => {
                exit_code = payload.code;
                break;
            }
            _ => {}
        }
    }

    *active.0.lock().map_err(|e| e.to_string())? = None;

    if exit_code != Some(0) {
        let _ = std::fs::remove_file(&out_path);
        return Err(extract_ffmpeg_error(&stderr_buf));
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

const FFMPEG_IMAGE_OUTPUT_FORMATS: &[&str] = &["avif"];

pub const VIDEO_FORMATS: &[&str] = &[
    // Modern
    "mp4", "mkv", "mov", "webm",
    // Common
    "avi", "m4v", "ogv", "gif", "ts", "flv",
    // Mobile
    "3gp", "f4v", "3g2",
    // Broadcast & Pro
    "mts", "mxf", "mpg", "vob", "wmv",
    // Legacy
    "asf", "divx", "rm", "apng",
];
pub const VIDEO_CODECS: &[&str] = &["h264", "h265", "vp9"];

fn video_codec_flag(codec: &str) -> Result<&'static str, String> {
    match codec {
        "h264" => Ok("libx264"),
        "h265" => Ok("libx265"),
        "vp9" => Ok("libvpx-vp9"),
        other => Err(format!("Unsupported codec: {}", other)),
    }
}

/// Returns a fixed video codec for container formats that do not support
/// user-selectable h264/h265/vp9 encoding.
fn video_codec_override(output_format: &str) -> Option<&'static str> {
    match output_format {
        "gif"             => Some("gif"),
        "apng"            => Some("apng"),
        "ogv"             => Some("libtheora"),
        "mpg"             => Some("mpeg2video"),
        "vob"             => Some("mpeg2video"),
        "wmv" | "asf"     => Some("wmv2"),   // may fail if FFmpeg not built with wmv2
        "rm"              => Some("rv10"),    // may fail if FFmpeg not built with rv10
        _                 => None,
    }
}

/// Audio codec to embed in a video container.
fn audio_codec_for_video_container(output_format: &str) -> &'static str {
    match output_format {
        "webm"                     => "libopus",
        "ogv"                      => "libvorbis",
        "avi" | "flv" | "divx"     => "libmp3lame",
        "vob"                      => "ac3",
        "wmv" | "asf"              => "wmav2", // may fail
        "rm"                       => "ac3",
        "mxf"                      => "pcm_s16le",
        "mpg"                      => "mp2",
        "gif" | "apng"             => "",      // no audio stream
        _                          => "aac",
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn convert_video(
    app: &tauri::AppHandle,
    active: &crate::ActiveConversion,
    input_path: &str,
    output_format: &str,
    output_path: &str,
    codec: Option<&str>,
    resolution_width: Option<u32>,
    resolution_height: Option<u32>,
    file_id: &str,
) -> Result<ConversionResult, String> {
    if !std::path::Path::new(input_path).exists() {
        return Err(format!("Input file not found: {}", input_path));
    }

    if !VIDEO_FORMATS.contains(&output_format) {
        return Err(format!(
            "Unsupported video format: {}. Allowed: {}",
            output_format,
            VIDEO_FORMATS.join(", ")
        ));
    }

    // Validate user codec only for formats without a fixed encoder override
    if let Some(c) = codec {
        if video_codec_override(output_format).is_none() && !VIDEO_CODECS.contains(&c) {
            return Err(format!(
                "Unsupported codec: {}. Allowed: {}",
                c,
                VIDEO_CODECS.join(", ")
            ));
        }
    }

    let input_size = std::fs::metadata(input_path)
        .map_err(|e| format!("Failed to read input metadata: {}", e))?
        .len();

    let mut args = vec![
        "-y".to_string(),
        "-i".to_string(),
        input_path.to_string(),
        "-progress".to_string(),
        "pipe:1".to_string(),
        "-nostats".to_string(),
    ];

    // Video codec: use format override first, then user selection
    let (vcodec, user_selected_vp9) = if let Some(fixed) = video_codec_override(output_format) {
        (fixed.to_string(), false)
    } else {
        let codec_name = codec.unwrap_or("h264");
        let flag = video_codec_flag(codec_name)?;
        (flag.to_string(), codec_name == "vp9")
    };

    if !vcodec.is_empty() {
        args.push("-c:v".to_string());
        args.push(vcodec);
    }

    // VP9 constant-quality mode requires -b:v 0
    if user_selected_vp9 {
        args.push("-b:v".to_string());
        args.push("0".to_string());
    }

    // Resolution scale filter
    match (resolution_width, resolution_height) {
        (Some(w), Some(h)) => {
            args.push("-vf".to_string());
            args.push(format!("scale={}:{}", w, h));
        }
        (Some(w), None) => {
            args.push("-vf".to_string());
            args.push(format!("scale={}:-2", w));
        }
        (None, Some(h)) => {
            args.push("-vf".to_string());
            args.push(format!("scale=-2:{}", h));
        }
        (None, None) => {}
    }

    // Audio codec (empty string = no audio track)
    let audio_codec = audio_codec_for_video_container(output_format);
    if audio_codec.is_empty() {
        args.push("-an".to_string());
    } else {
        args.push("-c:a".to_string());
        args.push(audio_codec.to_string());
    }

    args.push(output_path.to_string());

    let (mut rx, child) = app
        .shell()
        .sidecar("ffmpeg")
        .map_err(|e| e.to_string())?
        .args(&args)
        .spawn()
        .map_err(|e| e.to_string())?;

    *active.0.lock().map_err(|e| e.to_string())? = Some(child);

    let mut stderr_buf = String::new();
    let mut exit_code: Option<i32> = None;
    let mut duration_us: Option<u64> = None;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line) => {
                let text = String::from_utf8_lossy(&line);
                if let (Some(dur), Some(pos)) = (duration_us, parse_out_time_us(&text)) {
                    if dur > 0 {
                        let percent = ((pos as f64 / dur as f64) * 100.0).min(99.0) as f32;
                        let _ = app.emit(
                            "conversion-progress",
                            ProgressPayload {
                                id: file_id.to_string(),
                                percent,
                            },
                        );
                    }
                }
            }
            CommandEvent::Stderr(line) => {
                let text = String::from_utf8_lossy(&line);
                if duration_us.is_none() {
                    duration_us = parse_duration_us(&text);
                }
                stderr_buf.push_str(&text);
            }
            CommandEvent::Terminated(payload) => {
                exit_code = payload.code;
                break;
            }
            _ => {}
        }
    }

    *active.0.lock().map_err(|e| e.to_string())? = None;

    if exit_code != Some(0) {
        let _ = std::fs::remove_file(output_path);
        return Err(extract_ffmpeg_error(&stderr_buf));
    }

    let output_size = std::fs::metadata(output_path)
        .map_err(|e| format!("Failed to read output metadata: {}", e))?
        .len();

    Ok(ConversionResult {
        output_path: output_path.to_string(),
        input_size,
        output_size,
    })
}

pub async fn convert_image(
    app: &tauri::AppHandle,
    input_path: &str,
    output_format: &str,
    output_path: &str,
    resize: Option<(Option<u32>, Option<u32>)>,
) -> Result<ConversionResult, String> {
    if !std::path::Path::new(input_path).exists() {
        return Err(format!("Input file not found: {}", input_path));
    }

    if !FFMPEG_IMAGE_OUTPUT_FORMATS.contains(&output_format) {
        return Err(format!(
            "ffmpeg image converter only handles: {}",
            FFMPEG_IMAGE_OUTPUT_FORMATS.join(", ")
        ));
    }

    let input_size = std::fs::metadata(input_path)
        .map_err(|e| format!("Failed to read input metadata: {}", e))?
        .len();

    let mut args = vec!["-y".to_string(), "-i".to_string(), input_path.to_string()];

    if let Some((w, h)) = resize {
        let scale = match (w, h) {
            (Some(w), Some(h)) => format!("scale={}:{}", w, h),
            (Some(w), None) => format!("scale={}:-2", w),
            (None, Some(h)) => format!("scale=-2:{}", h),
            (None, None) => unreachable!(),
        };
        args.push("-vf".to_string());
        args.push(scale);
    }

    args.push(output_path.to_string());

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
        let _ = std::fs::remove_file(output_path);
        return Err(extract_ffmpeg_error(&stderr_buf));
    }

    let output_size = std::fs::metadata(output_path)
        .map_err(|e| format!("Failed to read output metadata: {}", e))?
        .len();

    Ok(ConversionResult {
        output_path: output_path.to_string(),
        input_size,
        output_size,
    })
}

#[cfg(test)]
mod video_tests {
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

    // ── Format / codec validation ─────────────────────────────────────────────

    #[test]
    fn test_video_formats_list() {
        for fmt in &["mp4", "mkv", "webm", "mov", "3gp", "avi", "ogv", "gif", "ts", "flv", "mpg", "wmv"] {
            assert!(VIDEO_FORMATS.contains(fmt), "{} should be in VIDEO_FORMATS", fmt);
        }
    }

    #[test]
    fn test_video_codec_override() {
        assert_eq!(video_codec_override("gif"), Some("gif"));
        assert_eq!(video_codec_override("apng"), Some("apng"));
        assert_eq!(video_codec_override("ogv"), Some("libtheora"));
        assert_eq!(video_codec_override("mpg"), Some("mpeg2video"));
        assert_eq!(video_codec_override("vob"), Some("mpeg2video"));
        assert_eq!(video_codec_override("wmv"), Some("wmv2"));
        assert_eq!(video_codec_override("mp4"), None);
        assert_eq!(video_codec_override("mkv"), None);
    }

    #[test]
    fn test_audio_codec_for_video_container() {
        assert_eq!(audio_codec_for_video_container("webm"), "libopus");
        assert_eq!(audio_codec_for_video_container("ogv"), "libvorbis");
        assert_eq!(audio_codec_for_video_container("avi"), "libmp3lame");
        assert_eq!(audio_codec_for_video_container("gif"), "");
        assert_eq!(audio_codec_for_video_container("apng"), "");
        assert_eq!(audio_codec_for_video_container("mpg"), "mp2");
        assert_eq!(audio_codec_for_video_container("vob"), "ac3");
        assert_eq!(audio_codec_for_video_container("mp4"), "aac");
    }

    #[test]
    fn test_video_codecs_list() {
        assert!(VIDEO_CODECS.contains(&"h264"));
        assert!(VIDEO_CODECS.contains(&"h265"));
        assert!(VIDEO_CODECS.contains(&"vp9"));
    }

    #[test]
    fn test_codec_flag_h264() {
        assert_eq!(video_codec_flag("h264").unwrap(), "libx264");
    }

    #[test]
    fn test_codec_flag_h265() {
        assert_eq!(video_codec_flag("h265").unwrap(), "libx265");
    }

    #[test]
    fn test_codec_flag_vp9() {
        assert_eq!(video_codec_flag("vp9").unwrap(), "libvpx-vp9");
    }

    #[test]
    fn test_codec_flag_unknown() {
        assert!(video_codec_flag("xvid").is_err());
    }

    #[test]
    fn test_audio_codec_webm() {
        assert_eq!(audio_codec_for_video_container("webm"), "libopus");
    }

    #[test]
    fn test_audio_codec_mp4() {
        assert_eq!(audio_codec_for_video_container("mp4"), "aac");
    }

    #[test]
    fn test_audio_codec_mkv() {
        assert_eq!(audio_codec_for_video_container("mkv"), "aac");
    }

    // ── Integration: real FFmpeg ──────────────────────────────────────────────

    fn create_minimal_mp4(path: &PathBuf) {
        let ffmpeg = match ffmpeg_bin() {
            Some(p) => p,
            None => return,
        };
        // Generate a 1-second color video with silent audio
        std::process::Command::new(&ffmpeg)
            .args([
                "-y",
                "-f",
                "lavfi",
                "-i",
                "color=black:size=64x64:rate=1:duration=1",
                "-f",
                "lavfi",
                "-i",
                "anullsrc=r=44100:cl=mono",
                "-t",
                "1",
                "-c:v",
                "libx264",
                "-c:a",
                "aac",
                "-shortest",
                path.to_str().unwrap(),
            ])
            .output()
            .ok();
    }

    #[test]
    fn test_mp4_to_mkv() {
        let ffmpeg = match ffmpeg_bin() {
            Some(p) => p,
            None => return,
        };
        let input = std::env::temp_dir().join("verto_test_video_input.mp4");
        create_minimal_mp4(&input);
        if !input.exists() {
            return;
        }

        let output = std::env::temp_dir().join("verto_test_video_mp4_to_mkv.mkv");
        let status = std::process::Command::new(&ffmpeg)
            .args([
                "-y",
                "-i",
                input.to_str().unwrap(),
                "-c:v",
                "libx264",
                "-c:a",
                "aac",
                output.to_str().unwrap(),
            ])
            .status()
            .expect("ffmpeg failed");

        assert!(status.success());
        assert!(output.exists());
        let _ = std::fs::remove_file(&output);
        let _ = std::fs::remove_file(&input);
    }

    #[test]
    fn test_nonexistent_video_input() {
        let ffmpeg = match ffmpeg_bin() {
            Some(p) => p,
            None => return,
        };
        let status = std::process::Command::new(&ffmpeg)
            .args(["-y", "-i", "/nonexistent/video.mp4", "/tmp/out.mkv"])
            .status()
            .expect("ffmpeg process failed");
        assert!(!status.success());
    }
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
        for fmt in &[
            "mp3", "aac", "m4a", "opus", "ogg", "flac", "wav", "aiff", "aif",
            "wv", "ape", "tta", "caf", "au", "ac3", "eac3", "mp2", "mka",
            "wma", "spx",
        ] {
            assert!(ALLOWED_FORMATS.contains(fmt), "{} should be allowed", fmt);
        }
    }

    #[test]
    fn test_audio_codec_for_format() {
        assert_eq!(audio_codec_for_format("mp3"), Ok("libmp3lame"));
        assert_eq!(audio_codec_for_format("flac"), Ok("flac"));
        assert_eq!(audio_codec_for_format("wav"), Ok("pcm_s16le"));
        assert_eq!(audio_codec_for_format("aiff"), Ok("pcm_s16be"));
        assert_eq!(audio_codec_for_format("aif"), Ok("pcm_s16be"));
        assert_eq!(audio_codec_for_format("opus"), Ok("libopus"));
        assert_eq!(audio_codec_for_format("ogg"), Ok("libvorbis"));
        assert_eq!(audio_codec_for_format("ac3"), Ok("ac3"));
        assert_eq!(audio_codec_for_format("mp2"), Ok("mp2"));
        assert!(audio_codec_for_format("unknown_fmt").is_err());
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
