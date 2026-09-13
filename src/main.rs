mod abbreviate;
mod cli;

use clap::Parser;
use crate::abbreviate::abbreviate;
use crate::cli::Cli;

fn main() {
    let words = Cli::parse().words;
    println!("\x1b[31m{}\x1b[0m", abbreviate(words));
}
