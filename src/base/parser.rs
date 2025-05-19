use crate::csv::OutputFormat;
use std::path::Path;

// 判断文件是否存在
pub fn input_file_exists(filename: &str) -> Result<String, String> {
    // 创建一个Path对象，传入文件名
    if Path::new(filename).exists() {
        // 如果文件存在，返回文件名
        Ok(filename.to_string())
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
