use crate::com::{CmdExecutor, input_exists, parse_input_path, parse_text_sign_format};
use crate::text::{text_generate_process, text_sign_process, text_verify_process};
use clap::Parser;
use std::fmt::Display;
use std::fs;
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

impl CmdExecutor for TextOpt {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextOpt::Sign(opt) => opt.execute().await?,
            TextOpt::Verify(opt) => opt.execute().await?,
            TextOpt::GenerateKey(opt) => opt.execute().await?,
        }
        Ok(())
    }
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

impl CmdExecutor for TextSignOpt {
    async fn execute(self) -> anyhow::Result<()> {
        let sign = text_sign_process(&self.input, &self.key, self.format)?;
        println!("{}", sign);
        Ok(())
    }
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

impl CmdExecutor for TextVerifyOpt {
    async fn execute(self) -> anyhow::Result<()> {
        let verify = text_verify_process(&self.input, &self.key, self.format, &self.sign)?;
        println!("{}", verify);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(long, value_parser = parse_input_path)]
    pub output: PathBuf,
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = text_generate_process(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0])?;
                fs::write(name.join("ed25519.pk"), &key[1])?;
            }
        }
        Ok(())
    }
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
