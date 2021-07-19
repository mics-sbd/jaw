use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

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

pub struct InitHandler {}

impl InitHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) -> Result<(), Error> {
        parse_command("pwsh", &resolve_script("init.ps1"))?;
        Ok(())
    }
}
