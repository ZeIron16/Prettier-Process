use clap::{Command, Arg, ArgMatches, ArgAction};
use std::sync::mpsc;

use crate::display::MyApp;
use crate::dwn;
use crate::comp;

pub fn build_cli() -> Command {
    Command::new("Chuka")
        .no_binary_name(true)
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(false)
        .disable_help_subcommand(true)
        .subcommand(
            Command::new("clear")
                .about("Clear the terminal")
        )
        .subcommand(
            Command::new("help")
                .about("Display help menu")
        )
        .subcommand(
            Command::new("exit")
                .about("Leave the application")
        )
        .subcommand(
            Command::new("dwn")
                .about("Download video or audio from an URL")
                .arg(Arg::new("url")
                    .help("URL of the video")
                    .required(true)
                    .index(1))
                .arg(Arg::new("audio")
                    .short('a')
                    .long("audio")
                    .help("Dowload only mp3")
                    .action(ArgAction::SetTrue))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .help("Name of the file")
                    .value_name("file"))
        )
        .subcommand(
            Command::new("compress")
            .about("Compress your file or folder with the optimal extension")
            .arg(Arg::new("input")
                    .help("Path of the file or folder to compress")
                    .required(true)
                    .index(1))
            .arg(Arg::new("output")
                    .help("Path of the output (/!\\ Do not add file extension)")
                    .required(true)
                    .index(2))
            .arg(Arg::new("max")
                    .short('m')
                    .long("max")
                    .help("Maximal compression: this would take way longer and the extension may be different")
                    .action(ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("decompress")
            .about("Decompress your file or folder")
            .arg(Arg::new("input")
                    .help("Path of the file or folder to decompress")
                    .required(true)
                    .index(1))
        )
}

// Handler de commandes avec clap
pub fn handle_command(matches: ArgMatches, app_state: &mut MyApp) -> Result<String, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("clear", _)) => {
            app_state.output.clear();
            Ok("Terminal cleared".to_string())
        }
        Some(("help", _)) => {
            let mut help_output = Vec::new();
            build_cli().write_help(&mut help_output)?;
            Ok(String::from_utf8(help_output)?)
        }
        Some(("exit", _)) => {
            std::process::exit(0);
        }
        Some(("dwn", args)) => {
            let url = args.get_one::<String>("url").unwrap();
            let output = args.get_one::<String>("output");
            let audio = args.get_flag("audio");
            
            let (tx, rx) = mpsc::channel::<f32>();
            app_state.download_rx = Some(rx);
            app_state.downloading = true;
            app_state.download_progress = 0.0;

            match dwn::download(url, audio, output, tx) {
                Ok(()) => Ok("Downloading ...".to_string()),
                Err(e) => Ok(format!("Failed to start download: {e}"))
            }
        }
        Some(("compress", args)) => {
            let input = args.get_one::<String>("input").unwrap();
            let output = args.get_one::<String>("output").unwrap();
            let max = args.get_flag("max");
            comp::compress(input, output, max)?;
            Ok(format!("\nCompression of '{}' to '{}' successful.\n", input, output))
        }
        Some(("decompress", args)) => {
            let input = args.get_one::<String>("input").unwrap();
            comp::decompress(input)?;
            Ok(format!("\nDecompression successful.\n"))
        }
        _ => {
            Ok("Unknown command. Try help to see the command list.".to_string())
        }
    }
}