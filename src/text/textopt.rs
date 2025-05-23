use crate::com::{input_exists, parse_input_path, parse_text_sign_format};
use clap::Parser;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Debug, Parser)]
pub enum TextOpt {
    #[command(about = "")]
    Sign(TextSignOpt),

    #[command(about = "")]
    Verify(TextVerifyOpt),

    #[command(about = "")]
    GenerateKey(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpt {
    #[arg(short, long, value_parser = input_exists, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = input_exists)]
    pub key: String,

    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpt {
    #[arg(short, long, value_parser = input_exists, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = input_exists)]
    pub key: String,

    #[arg(long)]
    pub sign: String,

    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(long, value_parser = parse_input_path)]
    pub output: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("invalid format".to_string())),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
