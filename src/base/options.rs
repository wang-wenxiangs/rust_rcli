use crate::csv::options::CsvOption;
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
    Csv(CsvOption),
}
