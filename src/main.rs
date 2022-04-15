use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "easy",
    author = "Caleb Schoepp <hey@calebschoepp.com>",
    about = "An easy to understand Wasm interpreter"
)]
struct CLI {
    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(name = "run", about = "Run a web assembly module")]
    Run(RunOptions),
}

#[derive(Debug, StructOpt)]
struct RunOptions {
    #[structopt(parse(from_os_str), help = "Path to a Wasm module")]
    module: PathBuf,
}

fn main() {
    let cli = CLI::from_args();
    match cli.cmd {
        SubCommand::Run(options) => run(options),
    }
}

// TODO: Return result error of some kind
fn run(options: RunOptions) {
    // TODO: Call out to library here
    println!("{:?}", options.module)
}
