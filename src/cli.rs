use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, name = "nightmare-says")]
pub struct Cli {
    pub words: Vec<String>,
}