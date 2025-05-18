use crate::csvoptions::Player;
use anyhow::Result;
use csv::Reader;
use std::fs;
use std::fs::File;

pub fn csv_process(input: &str, output: &str) -> Result<()> {
    let mut reader: Reader<File> = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
