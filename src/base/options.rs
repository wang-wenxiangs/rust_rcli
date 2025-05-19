use crate::csv::CsvOpts;
use crate::genpass::GenPassOpts;
use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "rust_rcli", author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a password")]
    GenPass(GenPassOpts),
}
