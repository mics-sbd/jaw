extern crate structopt;

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
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

fn parse_command(cmd: &str, arg: &str) -> Result<(), Error> {
    let stdout = Command::new(cmd)
        .stdout(Stdio::piped())
        .args(&["-c", arg])
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .for_each(|line| println!("{:#?}", line.unwrap()));

    Ok(())
}

fn main() {
    let opt = Cli::from_args();
    match opt.action {
        Action::Run => parse_command("pwsh", "/workspaces/just-add-water/deploy.ps1"),
        Action::Add => parse_command("ls", "-la"),
        Action::Init => parse_command("pwsh", "/workspaces/just-add-water/init.ps1"),
        _ => Err(Error::new(ErrorKind::Other, "Unrecognized")),
    };
}
