use easy::{Module, New};
use std::{fs::File, io::Read, path::PathBuf};
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

fn run(options: RunOptions) {
    let extension = options.module.extension();
    if extension.is_none() || extension.unwrap() != "wasm" {
        println!("Incorrect file extension");
        // TODO: Return result error of some kind
        return;
    }
    if !options.module.exists() {
        println!("Wasm module not found at path");
        // TODO: Return result error of some kind
        return;
    }

    // TODO: Slice or vector or bytes more efficient?
    let bytes = read_file(options.module);

    let module = Module::new(&bytes as &[u8]);
    println!("{:?}", module);
}

fn read_file(path: PathBuf) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_end(&mut data).expect("Unable to read data");
    data
}

// TODO: Write some integration tests that run the entire CLI
