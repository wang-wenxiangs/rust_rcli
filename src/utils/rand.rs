use anyhow::Result;
use std::fs::File;
use std::io::Read;

pub fn get_read(input: &str) -> Result<Box<dyn Read>> {
    let read: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(read)
}
