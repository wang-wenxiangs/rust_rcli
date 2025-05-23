use crate::base64::Base64Format;
use crate::utils::get_read;
use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use std::io::Read;

pub fn encode_process(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut read = get_read(input)?;
    let mut buf = Vec::new();
    read.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    Ok(encoded)
}

pub fn decode_process(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut read = get_read(input)?;
    let mut buf = String::new();
    read.read_to_string(&mut buf)?;
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}
