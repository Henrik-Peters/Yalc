use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;

use crate::config::Config;

/// Load the config file from disk and parse the config.
/// This function will also validate the config before parsing.
/// The config file will be decoded with UTF-8.
pub fn load_config(path: &Path) -> Result<Config, io::Error> {
    println!("Loading config from: {}", &path.display());
    let config_lines: Vec<String> = load_config_file_lines(&path)?;

    println!("config_lines: {:?}", config_lines);

    Err(io::Error::new(ErrorKind::Other, "Not implemented"))
}

/// Load the config file lines. Each line will a string in the result vec.
/// This function assumes that LF ("\n") or CRLF ("\r\n") is used for line separation.
/// For string decoding UTF-8 is used.
fn load_config_file_lines(path: &Path) -> Result<Vec<String>, io::Error> {
    let content: String = fs::read_to_string(path)?;
    Ok(content
        .split_terminator('\n') //Use split_terminator for handling of \r\n or \n
        .map(|line| line.to_string())
        .collect())
}
