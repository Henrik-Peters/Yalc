use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;

use crate::config::Config;
use crate::config::toml_lexer::Lexer;
use crate::config::toml_lexer::Token;

/// Load the config file from disk and parse the config.
/// This function will also validate the config before parsing.
/// The config file will be decoded with UTF-8.
pub fn load_config(path: &Path) -> Result<Config, io::Error> {
    println!("Loading config from: {}", &path.display());
    let config_content: String = load_config_file_lines(&path)?;

    let mut lexer = Lexer::new(&config_content);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == Token::EOF {
            break; //Exit loop when EOF is reached
        }
    }

    Err(io::Error::new(ErrorKind::Other, "Not implemented"))
}

/// Load the config file lines. Each line will a string in the result vec.
/// This function assumes that LF ("\n") or CRLF ("\r\n") is used for line separation.
/// For string decoding UTF-8 is used.
fn load_config_file_lines(path: &Path) -> Result<String, io::Error> {
    let content: String = fs::read_to_string(path)?;
    Ok(content)
}
