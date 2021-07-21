extern crate structopt;

use std::io::Error;

use handlers::build_handler::{BuildHandler, BuildOptions};
use handlers::test_handler::{TestHandler, TestOptions};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    /// Action to take
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(about = "Builds current repo")]
    Build(BuildOptions),
    #[structopt(about = "Runs tests")]
    Test(TestOptions),
}

fn main() -> Result<(), Error> {
    let opt = Cli::from_args();

    // Load Handlers
    let build_handler = BuildHandler::new();
    let test_handler = TestHandler::new();

    // Dispatch
    match opt.subcommand {
        Subcommand::Build(options) => build_handler.run(options),
        Subcommand::Test(options) => test_handler.run(options),
    }
}
