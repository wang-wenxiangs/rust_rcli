use crate::csv::options::OutputFormat;

pub fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
