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
    Pinfo {pid: usize, #[arg(long)]json: bool, #[arg(long)]file: bool, #[arg(long)]all: bool},
} // Describes the command list and their arguments 

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: ComList,
}

/*
------------------------------------------------------------------------------------------------------------------------
Function handler:   -input:         a command line (allready parsed)
                    -output:        Result type (did it succed or not)
                    -description:   call the function associated to the command, forwarding the arguments
------------------------------------------------------------------------------------------------------------------------
*/
pub fn handler(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        ComList::List { json, file} => proc::list_proc(json, file),
        ComList::Pinfo { pid, json, file, all} => proc::pinfo(pid, json, file, all),
        ComList::Stats { json, file } => stats::statistics(json, file),
        ComList::Live { pid, json } => live::start(pid, json),
    }
}