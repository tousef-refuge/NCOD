pub mod cli;

use clap::Parser;
use crate::cli::Cli;

fn main() {
    let words = Cli::parse().words;
}
