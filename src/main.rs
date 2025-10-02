use clap::Parser;

mod cli;
mod live;
mod proc;
mod stats;

fn main() {
    let _cli = cli::Cli::parse();
    if let Err(returned) = cli::handler(_cli) {
        println!("Internal ERROR : {returned}\n");
    }
}