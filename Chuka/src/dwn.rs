use std::fs;
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Read};
use std::thread;
use std::sync::mpsc::Sender;
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

fn read_progress<R: Read + Send + 'static>(pipe: R, tx: Sender<f32>) {
    thread::spawn(move || {
        let reader = BufReader::new(pipe);
        for line in reader.lines().flatten() {
            if let Some(percent_token) = line.split_whitespace().find(|s| s.ends_with('%')) {
                let number = percent_token.trim_end_matches('%').replace(',', ".");
                if let Ok(val) = number.parse::<f32>() {
                    let _ = tx.send(val);
                }
            }
        }
    });
}

pub fn download(url: &str, audio_only: bool, output_file: Option<&String>, progress_tx: Sender<f32>) -> Result<(), Box<dyn std::error::Error>> {
    let ytdlp_path = prepare_binary()?;
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

    cmd.arg("--newline").arg("--no-color");
    cmd.arg(url);

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    if let Some(stdout) = child.stdout.take() {
        read_progress(stdout, progress_tx.clone());
    }

    if let Some(stderr) = child.stderr.take() {
        read_progress(stderr, progress_tx.clone());
    }

    let tx_final = progress_tx.clone();
    thread::spawn(move || {
        let _ = child.wait();
        let _ = tx_final.send(100.0);
    });

    Ok(())
}