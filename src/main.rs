extern crate structopt;

use structopt::StructOpt;
use std::process::Command;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    /// Action to take
    action: Action,

}

#[derive(Debug, StructOpt)]
enum Action {
    Run {

    }
}

fn parse_command(cmd: &str) -> Vec<u8> {
    let result = Command::new(cmd).output();
    match result {
        Ok (output) => return output.stdout,
        Err(e) => panic!("child command failed") 
    }
}

fn main() {
    let opt = Cli::from_args();

    match opt.action {
        Run => println!("{}", String::from_utf8_lossy(&parse_command("pwsh /home/zach/Source/Repos/just-add-water/deploy.ps1"))),
        _ => panic!("invalid subcommand")
    };
}
