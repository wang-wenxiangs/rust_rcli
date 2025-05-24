use crate::com::{CmdExecutor, parse_input_path};
use crate::httpserve::http_serve_process;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HttpOpt {
    #[command(about = "Start a httpserve server")]
    Serve(HttpServerOpt),
}

impl CmdExecutor for HttpOpt {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpOpt::Serve(opt) => opt.execute().await?,
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct HttpServerOpt {
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,

    #[clap(short, long, value_parser = parse_input_path, default_value = ".")]
    pub dir: PathBuf,
}

impl CmdExecutor for HttpServerOpt {
    async fn execute(self) -> anyhow::Result<()> {
        http_serve_process(self.dir, self.port).await?;
        Ok(())
    }
}
