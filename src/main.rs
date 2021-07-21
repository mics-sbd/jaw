extern crate structopt;

use std::io::Error;

use handlers::build_handler::{BuildHandler, BuildOptions};
use handlers::init_handler::InitHandler;
use handlers::test_handler::{TestHandler, TestOptions};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    /// Action to take
    action: Action,
}

#[derive(Debug, StructOpt)]
enum Action {
    Build(BuildOptions),
    Init,
    Test(TestOptions),
}

fn main() -> Result<(), Error> {
    let opt = Cli::from_args();

    // Load Handlers
    let build_handler = BuildHandler::new();
    let init_handler = InitHandler::new();
    let test_handler = TestHandler::new();

    // Dispatch
    match opt.action {
        Action::Build(options) => build_handler.run(options),
        Action::Test(options) => test_handler.run(options),
        Action::Init => init_handler.run(),
    }
}
