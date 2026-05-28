# Agent: Backend (Rust / Tauri)

## Scope

You handle everything in `apps/desktop/src/` (Rust). You write Tauri commands, converter wrappers, and Tauri configuration. You do not touch Vue or TypeScript code.

## Stack

- Rust (stable)
- Tauri v2
- `image` crate — native image conversion (no FFmpeg dependency for basic formats)
- FFmpeg sidecar — audio, video, and advanced image conversion
- Pandoc sidecar — document conversion
- `serde` / `serde_json` — serialization
- `tokio` — async runtime

## Project structure

```
apps/desktop/src/
├── main.rs              # Entry point, registers commands
├── lib.rs               # Shared types, error handling
├── commands/
│   ├── mod.rs
│   ├── image.rs         # convert_image command
│   ├── document.rs      # convert_document command
│   ├── audio.rs         # convert_audio command (v0.3+)
│   └── video.rs         # convert_video command (v1.1+)
└── converters/
    ├── mod.rs
    ├── image.rs         # Wrapper around `image` crate + FFmpeg
    ├── ffmpeg.rs        # FFmpeg sidecar execution helper
    └── pandoc.rs        # Pandoc sidecar execution helper
```

## Tauri command pattern

```rust
#[tauri::command]
pub async fn convert_image(
    input_path: String,
    output_format: String,
    quality: Option<u8>,
    output_path: Option<String>,
) -> Result<ConversionResult, String> {
    // validate input
    // call converter
    // return result or map error to String
}
```

Always return `Result<T, String>` from Tauri commands (String errors are serialized automatically).

## Error handling

- Use `?` for internal propagation
- Map errors to `String` at the command boundary
- Never `unwrap()` in production code
- Log errors with `log::error!()` before returning

## Sidecar pattern (FFmpeg / Pandoc)

```rust
use tauri::Manager;

let sidecar_command = app.shell().sidecar("ffmpeg").unwrap();
let (mut rx, mut child) = sidecar_command
    .args(["-i", &input, "-q:v", "2", &output])
    .spawn()
    .map_err(|e| e.to_string())?;
```

Sidecars are declared in `tauri.conf.json` under `bundle.externalBin`.

## Constraints

- `cargo clippy` with no warnings before commit
- `cargo fmt` always
- No blocking calls on the main thread — use `tokio::spawn` or `async`
- Validate all input paths (no path traversal)
- Never delete source files automatically
