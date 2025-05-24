use crate::base64::{decode_process, encode_process};
use crate::com::{CmdExecutor, input_exists, parser_base64_format};
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64Opt {
    #[command(name = "encode", about = "")]
    Encode(Base64EncodeOpt),

    #[command(name = "decode", about = "")]
    Decode(Base64DecodeOpt),
}

impl CmdExecutor for Base64Opt {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64Opt::Encode(opt) => opt.execute().await,
            Base64Opt::Decode(opt) => opt.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpt {
    #[arg(short, long, value_parser = input_exists, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parser_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64EncodeOpt {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = encode_process(&self.input, self.format)?;
        println!("{}", encode);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpt {
    #[arg(short, long, value_parser = input_exists, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parser_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64DecodeOpt {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = decode_process(&self.input, self.format)?;
        println!("{}", decode);
        Ok(())
    }
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!(format!("Invalid format: {}", s))),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
