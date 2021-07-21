extern crate structopt;

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use structopt::StructOpt;

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
        .for_each(|line| println!("{}", line.unwrap()));

    Ok(())
}

fn resolve_script(script_name: &str) -> String {
    let esp_root_dir = "~/ESP/ESP.Build/scripts";
    format!("{}/{}", esp_root_dir, script_name)
}

pub struct TestHandler {}

#[derive(Debug, StructOpt)]
pub struct TestOptions {
    #[structopt(short, long, default_value = "*")]
    filter: String,
}

impl TestHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, options: TestOptions) -> Result<(), Error> {
        let command = if options.filter != "" {
            let cmd = format!("test.ps1 -Filter {}", options.filter);
            cmd
        } else {
            let cmd = format!("test.ps1");
            cmd
        };

        parse_command("pwsh", &resolve_script(&command))?;
        Ok(())
    }
}
