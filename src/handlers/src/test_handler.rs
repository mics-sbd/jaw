extern crate structopt;

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio, exit};
use structopt::clap::arg_enum;
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

arg_enum! {
    #[derive(Debug)]
    pub enum ProjectType {
        LLVM,
        MSVC,
    }
}
#[derive(Debug, StructOpt)]
pub struct TestOptions {
    #[structopt(possible_values = &ProjectType::variants(), case_insensitive = true, default_value="LLVM")]
    project_type: ProjectType,
    #[structopt(short, long, default_value = "*")]
    filter: String,
    #[structopt(long, default_value = "0")]
    retries: i32,
    #[structopt(short, long)]
    release: bool,
    #[structopt(long)]
    asan: bool,
    #[structopt(long)]
    coverage: bool,
}

impl TestHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, test_options: TestOptions) -> Result<(), Error> {
        let mut command = format!(
            "test.ps1 -ProjectType {} -Filter {} -Retries {}",
            test_options.project_type, test_options.filter, test_options.retries
        );

        if test_options.release {
            &command.push_str(" -Release");
        }
        if test_options.asan {
            if test_options.coverage {
                println!("Asan and Coverage are mutually exclusive!");
                exit(1);
            }
            &command.push_str(" -Asan");
        }
        if test_options.coverage {
            if test_options.asan {
                println!("Asan and Coverage are mutually exclusive!");
                exit(1);
            }
            &command.push_str(" -Coverage");
        }

        parse_command("pwsh", &resolve_script(&command))?;
        Ok(())
    }
}
