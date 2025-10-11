use std::fs;
use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Stdio};

#[cfg(target_os = "linux")]
const YTDLP_BINARY: &[u8] = include_bytes!("../resources/yt-dlp_linux");

#[cfg(target_os = "windows")]
const YTDLP_BINARY: &[u8] = include_bytes!("../resources/yt-dlp.exe");

fn prepare_binary() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir();

    #[cfg(target_os = "windows")]
    let ytdlp_path = temp_dir.join("chuka_ytdlp.exe");

    #[cfg(not(target_os = "windows"))]
    let ytdlp_path = temp_dir.join("chuka_ytdlp");

    if !ytdlp_path.exists() {
        fs::write(&ytdlp_path, YTDLP_BINARY)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&ytdlp_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&ytdlp_path, perms)?;
        }
    }
    Ok(ytdlp_path)
}

pub fn download(
    url: &str,
    audio_only: bool,
    output_file: Option<&String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let ytdlp_path = match prepare_binary() {
        Ok(path) => path,
        Err(e) => return Ok(format!("Error while extracting yt-dlp: {}", e)),
    };

    let mut cmd = ProcessCommand::new(&ytdlp_path);

    if audio_only {
        cmd.arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--audio-quality")
            .arg("0");
    }

    if let Some(output) = output_file {
        cmd.arg("-o").arg(output);
    } else {
        cmd.arg("-o").arg("%(title)s.%(ext)s");
    }

    cmd.arg(url).stdout(Stdio::null()).stderr(Stdio::null());

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                Ok("Download finished successfully!".to_string())
            } else {
                Ok(format!("Download failed (exit code: {:?})", status.code()))
            }
        }
        Err(e) => Ok(format!("Execution error: {}", e)),
    }
}