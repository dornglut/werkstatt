#![forbid(unsafe_code)]

use std::io::{self, Write};

use clap::{CommandFactory, Parser};

#[derive(Debug, Parser)]
#[command(
    name = "werkstatt",
    version,
    about = "Human-first engineering workbench"
)]
struct Cli {}

fn main() {
    if let Err(error) = run() {
        let _ = writeln!(io::stderr().lock(), "werkstatt output failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let _ = Cli::parse();
    let mut output = io::stdout().lock();
    Cli::command().write_help(&mut output)?;
    writeln!(output)
}
