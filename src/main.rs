use clap::Parser;

mod cli;
mod live;
mod proc;
mod stats;
mod struct_proc;

/*
------------------------------------------------------------------------------------------------------------------------
Function main:  - input:        /
                - ouput:        /
                - description:  parse the given command and call the command handler to execute the right one
------------------------------------------------------------------------------------------------------------------------
*/
fn main() {
    let _cli = cli::Cli::parse();
    if let Err(returned) = cli::handler(_cli) {
        println!("Internal ERROR : {returned}\n");
    }
}