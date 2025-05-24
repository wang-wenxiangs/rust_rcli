use crate::base64::Base64Opt;
use crate::csv::CsvOpt;
use crate::genpass::GenPassOpt;
use crate::httpserve::HttpOpt;
use crate::text::TextOpt;
use clap::Parser;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}

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

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opt) => opt.execute().await,
            SubCommand::GenPass(opt) => opt.execute().await,
            SubCommand::Base64(opt) => opt.execute().await,
            SubCommand::Text(opt) => opt.execute().await,
            SubCommand::Http(opt) => opt.execute().await,
        }
    }
}
