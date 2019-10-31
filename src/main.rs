extern crate structopt;

use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    /// Action to take
    action: Action,
}

#[derive(Debug, StructOpt)]
enum Action {
    Run,
    Add,
    Remove,
    Init,
    User {},
}

fn parse_command(cmd: &str, arg: &str) -> Vec<u8> {
    let result = Command::new(cmd).args(&["-c", arg]).output();
    match result {
        Ok(output) => output.stdout,
        Err(e) => panic!("Corrupt Installation: {}", e),
    }
}

fn main() {
    let opt = Cli::from_args();

    match opt.action {
        Action::Run => println!(
            "{:}",
            String::from_utf8_lossy(&parse_command(
                "pwsh",
                "/workspaces/just-add-water/deploy.ps1"
            ))
        ),
        Action::Add => println!("{:}", String::from_utf8_lossy(&parse_command("ls", "-la"))),
        Action::Init => println!(
            "{:}",
            String::from_utf8_lossy(&parse_command(
                "pwsh",
                "/workspaces/just-add-water-init.ps1"
            ))
        ),
        _ => println!("Command Not Yet Implemented"),
    };
}
