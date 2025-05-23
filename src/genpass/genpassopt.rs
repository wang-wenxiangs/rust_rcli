use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
pub struct GenPassOpt {
    #[arg(long, default_value = "16")]
    pub length: u8,

    #[arg(long, default_value = "true")]
    pub uppercase: bool,

    #[arg(long, default_value = "true")]
    pub lowercase: bool,

    #[arg(long, default_value = "true")]
    pub numbers: bool,

    #[arg(long, default_value = "true")]
    pub symbols: bool,
}
