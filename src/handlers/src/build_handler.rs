use std::env;
use std::fs;
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
        .for_each(|line| println!("{:#}", line.unwrap()));

    Ok(())
}

fn resolve_script(script_name: &str) -> String {
    let esp_root_dir = "~/ESP/ESP.Build/scripts";
    format!("{}/{}", esp_root_dir, script_name)
}

fn detect_support() -> Result<String, Error> {
    loop {
        let current_dir = env::current_dir()?;
        for entry in fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();
            if path.file_name().unwrap() == ".git" {
                return Ok(String::from(current_dir.to_str().unwrap()));
            }
        }
        env::set_current_dir("..")?;
    }
}

pub struct BuildHandler {
    context: String,
}

impl BuildHandler {
    pub fn new() -> Self {
        Self {
            context: detect_support().unwrap_or(String::from("")),
        }
    }

    pub fn run(self) -> Result<(), Error> {
        parse_command("pwsh", &resolve_script(&format!("symlink.ps1 -targetPath {}", self.context)))?;
        parse_command("pwsh", &resolve_script("build.ps1"))?;
        parse_command("pwsh", &resolve_script("unsymlink.ps1"))?;
        Ok(())
    }
}
