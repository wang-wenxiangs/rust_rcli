use crate::text::textsign::{Ed25519Sign, Ed255195Verify, KeyGenerator, Keyloader};
use crate::text::{TextBlake3, TextSign, TextSignFormat, TextVerify};
use crate::utils::get_read;
use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
pub fn text_sign_process(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut read = get_read(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let blake = TextBlake3::load(key)?;
            blake.sign(&mut read)?
        }
        TextSignFormat::Ed25519 => {
            let sign = Ed25519Sign::load(key)?;
            sign.sign(&mut read)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

pub fn text_verify_process(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sign: &str,
) -> Result<bool> {
    let mut read = get_read(input)?;
    let signed = URL_SAFE_NO_PAD.decode(sign)?;
    let verifyed = match format {
        TextSignFormat::Blake3 => {
            let verifier = TextBlake3::load(key)?;
            verifier.verify(&mut read, &signed)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed255195Verify::load(key)?;
            verifier.verify(&mut read, &signed)?
        }
    };
    Ok(verifyed)
}

pub fn text_generate_process(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => TextBlake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Sign::generate(),
    }
}
