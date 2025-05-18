use crate::csvoptions::OutputFormat;

pub fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
