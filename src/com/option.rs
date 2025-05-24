use crate::base64::Base64Opt;
use crate::csv::CsvOpt;
use crate::genpass::GenPassOpt;
use crate::httpserve::HttpOpt;
use crate::text::TextOpt;
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
    Csv(CsvOpt),

    #[command(name = "genpass", about = "Generate a password")]
    GenPass(GenPassOpt),

    #[command(subcommand)]
    Base64(Base64Opt),

    #[command(subcommand)]
    Text(TextOpt),

    #[command(subcommand)]
    Http(HttpOpt),
}
