use crate::csv::options::OutputFormat;
use anyhow::Result;
use csv::Reader;
use std::fs;
use std::fs::File;

// 定义一个函数，用于处理CSV文件
pub fn csv_process(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // 从输入路径创建一个Reader
    let mut reader: Reader<File> = Reader::from_path(input)?;
    // 获取CSV文件的表头
    let headers = reader.headers()?.clone();
    // 创建一个容量为1024的向量，用于存储处理后的数据
    let mut ret = Vec::with_capacity(1024);
    // 遍历CSV文件的每一行
    for result in reader.records() {
        // 获取当前行的数据
        let value = result?;
        // 将表头和当前行的数据组合成一个serde_json::Value
        let json_value = headers
            .iter()
            .zip(value.iter())
            .collect::<serde_json::Value>();
        // 将组合后的数据添加到向量中
        ret.push(json_value);
    }
    // 根据输出格式，将向量转换为字符串
    let contents = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    // 将转换后的字符串写入输出文件
    fs::write(output, contents)?;
    // 返回Ok()
    Ok(())
}
