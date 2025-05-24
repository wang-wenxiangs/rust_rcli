mod base64;
mod com;
mod csv;
mod genpass;
mod httpserve;
mod text;
mod utils;

use crate::com::{CmdExecutor, Options};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // 解析命令行参数
    let option = Options::parse();
    // 匹配命令行参数中的子命令
    option.cmd.execute().await?;
    Ok(())
}

#[test]
fn my_test() {
    assert_eq!(1 + 1, 2);
}
