mod base;
mod csv;
mod genpass;
use crate::base::{Options, SubCommand};
use crate::csv::csv_process;
use crate::genpass::genpass_process;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let option = Options::parse();
    // 匹配命令行参数中的子命令
    match option.cmd {
        // 如果子命令是Csv
        SubCommand::Csv(csv_option) => {
            // 如果Csv命令行参数中有输出文件名，则使用该文件名，否则使用默认文件名
            let output = if let Some(output) = csv_option.output {
                output.clone()
            } else {
                format!("assets/output.{}", csv_option.format)
            };
            // 调用csv_process函数处理Csv文件
            csv_process(&csv_option.input, output, csv_option.format)?;
        }
        SubCommand::GenPass(genpass_option) => {
            genpass_process(
                genpass_option.length,
                genpass_option.uppercase,
                genpass_option.lowercase,
                genpass_option.numbers,
                genpass_option.symbols,
            )?;
        }
    }
    // 返回Ok(())表示程序执行成功
    Ok(())
}

#[test]
fn my_test() {
    assert_eq!(1 + 1, 2);
}
