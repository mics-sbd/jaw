extern crate structopt;

use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::exit;
use std::process::{Command, Stdio};
use structopt::StructOpt;
use structopt::clap::arg_enum;

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

arg_enum! {
    #[derive(Debug)]
    pub enum ProjectType {
        LLVM,
        MSVC,
    }
}
#[derive(Debug, StructOpt)]
pub struct BuildOptions {
    #[structopt(possible_values = &ProjectType::variants(), case_insensitive = true, default_value="LLVM")]
    project_type: ProjectType,
    #[structopt(short, long)]
    clean: bool,
    #[structopt(short, long)]
    release: bool,
    #[structopt(long)]
    asan: bool,
    #[structopt(long)]
    coverage: bool,
}

impl BuildHandler {
    pub fn new() -> Self {
        Self {
            context: detect_support().unwrap_or(String::from("")),
        }
    }

    pub fn run(self, build_options: BuildOptions) -> Result<(), Error> {

        let mut command = format!(
            "build.ps1 -ProjectType {} -IDE vs",
            build_options.project_type
        );

        if build_options.clean {
            &command.push_str(" -CleanBuild");
        }
        if build_options.release {
            &command.push_str(" -Release");
        }
        if build_options.asan {
            if build_options.coverage {
                println!("Asan and Coverage are mutually exclusive!");
                exit(1);
            }
            &command.push_str( " -Asan");
        }
        if build_options.coverage {
            if build_options.asan {
                println!("Asan and Coverage are mutually exclusive!");
                exit(1);
            }
            &command.push_str( " -Coverage");
        }

        parse_command(
            "pwsh",
            &resolve_script(&format!("symlink.ps1 -targetPath {}", self.context)),
        )?;
        parse_command("pwsh", &resolve_script(&command))?;
        parse_command("pwsh", &resolve_script("unsymlink.ps1"))?;
        Ok(())
    }
}
