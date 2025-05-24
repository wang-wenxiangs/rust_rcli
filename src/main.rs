mod base64;
mod com;
mod csv;
mod genpass;
mod httpserve;
mod text;
mod utils;

use crate::base64::{Base64Opt, decode_process, encode_process};
use crate::com::{Options, SubCommand};
use crate::csv::csv_process;
use crate::genpass::genpass_process;
use crate::httpserve::{HttpOpt, http_serve_process};
use crate::text::{
    TextOpt, TextSignFormat, text_generate_process, text_sign_process, text_verify_process,
};
use clap::Parser;
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
            let (password, score) = genpass_process(
                genpass_option.length,
                genpass_option.uppercase,
                genpass_option.lowercase,
                genpass_option.numbers,
                genpass_option.symbols,
            )?;
            println!("Password: {}", password);
            println!("Score: {}", score);
        }
        SubCommand::Base64(base64_option) => match base64_option {
            Base64Opt::Encode(opt) => {
                let encode = encode_process(&opt.input, opt.format)?;
                println!("{}", encode);
            }
            Base64Opt::Decode(opt) => {
                let decode = decode_process(&opt.input, opt.format)?;
                println!("{}", decode);
            }
        },
        SubCommand::Text(text_option) => match text_option {
            TextOpt::Sign(option) => {
                let sign = text_sign_process(&option.input, &option.key, option.format)?;
                println!("{}", sign);
            }
            TextOpt::Verify(option) => {
                let verify =
                    text_verify_process(&option.input, &option.key, option.format, &option.sign)?;
                println!("{}", verify);
            }
            TextOpt::GenerateKey(option) => {
                let key = text_generate_process(option.format)?;
                match option.format {
                    TextSignFormat::Blake3 => {
                        let name = option.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &option.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(http_option) => match http_option {
            HttpOpt::Serve(option) => {
                http_serve_process(option.dir, option.port).await?;
            }
        },
    }
    // 返回Ok(())表示程序执行成功
    Ok(())
}

#[test]
fn my_test() {
    assert_eq!(1 + 1, 2);
}
