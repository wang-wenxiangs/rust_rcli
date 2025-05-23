use crate::base64::Base64Format;
use crate::csv::OutputFormat;
use crate::text::TextSignFormat;
use std::path::{Path, PathBuf};

// 判断文件是否存在
pub fn input_exists(filename: &str) -> Result<String, String> {
    // 创建一个Path对象，传入文件名
    if filename == "-" || Path::new(filename).exists() {
        // 如果文件存在，返回文件名
        Ok(filename.into())
    } else {
        // 如果文件不存在，返回错误信息
        Err(format!("File {} does not exist", filename))
    }
}

// 解析格式字符串，返回OutputFormat或错误
pub fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // 将格式字符串解析为OutputFormat
    format.parse()
}

pub fn parser_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

pub fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

pub fn parse_input_path(path: &str) -> Result<PathBuf, String> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory".to_string())
    }
}
