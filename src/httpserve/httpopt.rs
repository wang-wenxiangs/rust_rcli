use crate::com::parse_input_path;
use clap::Parser;
use std::path::PathBuf;
#[derive(Debug, Parser)]
pub enum HttpOpt {
    #[command(about = "Start a httpserve server")]
    Serve(HttpServerOpt),
}

#[derive(Debug, Parser)]
pub struct HttpServerOpt {
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,

    #[clap(short, long, value_parser = parse_input_path, default_value = ".")]
    pub dir: PathBuf,
}
