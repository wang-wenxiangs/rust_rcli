use std::path::Path;

pub fn input_file_exists(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File {} does not exist", filename))
    }
}
