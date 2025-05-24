mod base64;
mod com;
mod csv;
mod genpass;
mod httpserve;
mod text;
mod utils;

pub use base64::{Base64Format, Base64Opt, decode_process, encode_process};
pub use com::{
    Options, SubCommand, input_exists, parse_input_path, parse_text_sign_format, parser_format,
};
pub use csv::{CsvOpt, OutputFormat, csv_process};
pub use genpass::{GenPassOpt, genpass_process};
pub use httpserve::{HttpOpt, http_serve_process};
pub use text::{
    TextOpt, TextSignFormat, text_generate_process, text_sign_process, text_verify_process,
};
pub use utils::get_read;
