mod player;
mod process;
use super::base::input_file_exists;
use clap::Parser;
pub use player::Player;
pub use process::csv_process;

#[derive(Debug, Parser)]
pub struct CsvOption {
    #[arg(short, long, value_parser = input_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "assets/output.json")]
    pub output: String,
}
