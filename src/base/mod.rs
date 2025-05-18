mod parser;
use crate::csvoptions::CsvOption;
use clap::Parser;
pub use parser::input_file_exists;

#[derive(Debug, Parser)]
#[command(name = "rust_rcli", author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csvoptions", about = "Convert CSV to JSON")]
    Csv(CsvOption),
}
