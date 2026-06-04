use serde::Serialize;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Serialize)]
pub struct ConversionResult {
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
}

const ALLOWED_FORMATS: &[&str] = &[
    // Core
    "html", "pdf", "docx", "md", "rst", "odt", "epub", // Text/markup - priority
    "txt", "tex", "adoc", "org", "rtf", "pptx", // Data
    "ipynb", "docbook", "json", "xml", // Niche
    "textile", "wiki", "dokuwiki", "muse", "man", "ms", "beamer", "tei", "fb2", "icml", "jira",
    "markua", "zimwiki", // Web presentations
    "s5", "slidy", "slideous", "revealjs",
];

/// Returns the Pandoc `-t` format name for a given file extension.
/// For most formats the name matches the extension; only exceptions need explicit mapping.
fn output_format_flag(format: &str) -> &str {
    match format {
        "txt" => "plain",
        "tex" => "latex",
        "adoc" => "asciidoc",
        "md" => "markdown",
        "wiki" => "mediawiki",
        "docbook" => "docbook5",
        "xml" => "jats",
        other => other,
    }
}

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

    if !std::path::Path::new(input_path).is_absolute() {
        return Err("Input path must be absolute".to_string());
    }
    if !std::path::Path::new(&out_path).is_absolute() {
        return Err("Output path must be absolute".to_string());
    }

    let input_size = std::fs::metadata(input_path)
        .map_err(|e| format!("Failed to read input metadata: {}", e))?
        .len();

    let (mut rx, _child) = app
        .shell()
        .sidecar("pandoc")
        .map_err(|e| e.to_string())?
        .args([
            input_path,
            "-t",
            output_format_flag(output_format),
            "-o",
            &out_path,
        ])
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

    /// Locate the downloaded Pandoc sidecar binary, whichever platform we're on.
    fn pandoc_bin() -> Option<PathBuf> {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("binaries");
        [
            "pandoc-x86_64-unknown-linux-gnu",
            "pandoc-aarch64-unknown-linux-gnu",
            "pandoc-x86_64-apple-darwin",
            "pandoc-aarch64-apple-darwin",
            "pandoc-x86_64-pc-windows-msvc.exe",
        ]
        .iter()
        .map(|name| base.join(name))
        .find(|p| p.exists() && p.metadata().map(|m| m.len() > 0).unwrap_or(false))
    }

    // ── Format validation ─────────────────────────────────────────────────────

    #[test]
    fn test_allowed_formats_accepted() {
        for fmt in &[
            "html", "docx", "md", "rst", "odt", "epub", "pdf", "tex", "org", "txt", "adoc", "rtf",
            "pptx", "ipynb", "docbook", "json", "xml", "wiki", "fb2", "revealjs",
        ] {
            assert!(ALLOWED_FORMATS.contains(fmt), "{} should be allowed", fmt);
        }
    }

    #[test]
    fn test_disallowed_formats_rejected() {
        for fmt in &["mp3", "png", "zip", ""] {
            assert!(
                !ALLOWED_FORMATS.contains(fmt),
                "{} should not be allowed",
                fmt
            );
        }
    }

    #[test]
    fn test_output_format_flag() {
        assert_eq!(output_format_flag("txt"), "plain");
        assert_eq!(output_format_flag("tex"), "latex");
        assert_eq!(output_format_flag("adoc"), "asciidoc");
        assert_eq!(output_format_flag("md"), "markdown");
        assert_eq!(output_format_flag("wiki"), "mediawiki");
        assert_eq!(output_format_flag("docbook"), "docbook5");
        assert_eq!(output_format_flag("xml"), "jats");
        // identity mappings
        assert_eq!(output_format_flag("html"), "html");
        assert_eq!(output_format_flag("pdf"), "pdf");
        assert_eq!(output_format_flag("docx"), "docx");
        assert_eq!(output_format_flag("pptx"), "pptx");
        assert_eq!(output_format_flag("org"), "org");
        assert_eq!(output_format_flag("rtf"), "rtf");
        assert_eq!(output_format_flag("revealjs"), "revealjs");
    }

    // ── Output path computation ───────────────────────────────────────────────

    #[test]
    fn test_output_path_replaces_extension() {
        let p = PathBuf::from("/tmp/note.md").with_extension("html");
        assert_eq!(p.to_str().unwrap(), "/tmp/note.html");
    }

    #[test]
    fn test_output_path_docx_to_md() {
        let p = PathBuf::from("/home/user/report.docx").with_extension("md");
        assert_eq!(p.to_str().unwrap(), "/home/user/report.md");
    }

    // ── Integration: real Pandoc sidecar ─────────────────────────────────────

    #[test]
    fn test_md_to_html() {
        let pandoc = match pandoc_bin() {
            Some(p) => p,
            None => return,
        };
        let input = fixture("sample.md");
        if !PathBuf::from(&input).exists() {
            return;
        }

        let output = std::env::temp_dir()
            .join("verto_test_pandoc_md_to_html.html")
            .to_string_lossy()
            .into_owned();

        let status = std::process::Command::new(&pandoc)
            .args([&input, "-o", &output])
            .status()
            .expect("failed to run pandoc");

        assert!(status.success(), "pandoc md→html failed");
        assert!(PathBuf::from(&output).exists());
        let content = std::fs::read_to_string(&output).unwrap();
        assert!(content.contains('<'), "output should contain HTML tags");
        let _ = std::fs::remove_file(&output);
    }

    #[test]
    fn test_md_to_docx() {
        let pandoc = match pandoc_bin() {
            Some(p) => p,
            None => return,
        };
        let input = fixture("sample.md");
        if !PathBuf::from(&input).exists() {
            return;
        }

        let output = std::env::temp_dir()
            .join("verto_test_pandoc_md_to_docx.docx")
            .to_string_lossy()
            .into_owned();

        let status = std::process::Command::new(&pandoc)
            .args([&input, "-o", &output])
            .status()
            .expect("failed to run pandoc");

        assert!(status.success(), "pandoc md→docx failed");
        assert!(PathBuf::from(&output).exists());
        // DOCX is a ZIP archive, verify PK magic bytes
        let bytes = std::fs::read(&output).unwrap();
        assert_eq!(&bytes[..2], b"PK", "docx should be a valid ZIP/OOXML");
        let _ = std::fs::remove_file(&output);
    }

    #[test]
    fn test_nonexistent_input_fails() {
        let pandoc = match pandoc_bin() {
            Some(p) => p,
            None => return,
        };

        let output = std::env::temp_dir()
            .join("verto_test_pandoc_nonexistent.html")
            .to_string_lossy()
            .into_owned();

        let status = std::process::Command::new(&pandoc)
            .args(["/nonexistent/path/file.md", "-o", &output])
            .status()
            .expect("failed to run pandoc");

        assert!(!status.success(), "pandoc should fail on nonexistent input");
    }
}
