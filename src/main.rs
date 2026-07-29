#![forbid(unsafe_code)]

use clap::{CommandFactory, Parser};

#[derive(Debug, Parser)]
#[command(
    name = "werkstatt",
    version,
    about = "Human-first engineering workbench"
)]
struct Cli {}

fn main() {
    let _ = Cli::parse();
    Cli::command().print_help().expect("stdout is available");
    println!();
}
