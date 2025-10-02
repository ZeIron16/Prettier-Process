use clap::Parser;
use clap::Subcommand;

use crate::live;
use crate::proc;
use crate::stats;

#[derive(Subcommand)]
enum ComList {
    List {#[arg(long)]json: bool, #[arg(long)]file: bool},
    Stats {#[arg(long)]json: bool, #[arg(long)]file: bool},
    Live {pid: usize, #[arg(long)]json: bool},
    Pinfo {pid: usize, #[arg(long)]json: bool, #[arg(long)]file: bool},
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: ComList,
}

pub fn handler(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        ComList::List { json, file} => proc::list_proc(json, file),
        ComList::Pinfo { pid, json, file} => proc::pinfo(pid, json, file),
        ComList::Stats { json, file } => stats::statistics(json, file),
        ComList::Live { pid, json } => live::start(pid, json),
    }
}