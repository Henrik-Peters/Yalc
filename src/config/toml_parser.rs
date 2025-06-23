use std::collections::HashMap;
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
    let config_content: String = load_config_file_content(&path)?;

    //Collect all tokens and store in a vector
    let mut lexer = Lexer::new(&config_content);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let token = lexer.next_token();
        tokens.push(token);

        if tokens[tokens.len() - 1] == Token::EOF {
            break; //Exit loop when EOF is reached
        }
    }

    //Perform the parsing of the token list
    let mut parser = Parser::new(tokens);

    loop {
        let token = parser.next_token();

        match token {
            Some(token) => println!("{:?}", token),
            None => {
                break; //Exit loop when EOF is reached
            }
        }
    }

    Err(io::Error::new(ErrorKind::Other, "Not implemented"))
}

/// Load the config file content. Will return an error if the file does not exist.
/// This function assumes that the content of the file is encoded with UTF-8.
fn load_config_file_content(path: &Path) -> Result<String, io::Error> {
    let content: String = fs::read_to_string(path)?;
    Ok(content)
}

/// The root table of the toml file (outside of any section)
pub type TopLevelTable = Table;

/// Toml collection of key-value pairs - we use HashMap collection
pub type Table = HashMap<Key, Value>;

/// Name or identifier of the key-value pair
type Key = String;

#[derive(Debug, PartialEq)]
pub enum Value {
    /// Represents text with a String
    String(String),

    /// Represents whole numbers
    Integer(i64),

    /// Represents floating point numbers
    Float(f64),

    /// Represents true or false
    Bool(bool),

    /// Represents Timestamps - simple ISO 8601 string assumed
    DateTime(String),

    /// Represents an indexed list with values
    Array(Vec<Value>),

    /// Represents a toml collection of key-value pairs
    Table(Table),
}

pub struct Parser {
    /// Vector with all toml tokens provided by the lexer
    tokens: Vec<Token>,

    /// Index of the next token that will be processed
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            pos: 0,
        }
    }

    /// Retrieves the next token from the input token list
    ///
    /// The 'next_token()' function returns the next token which
    /// should be processed by the parser. The pos index is used
    /// to find the next token in the input list.
    ///
    /// # Returns
    /// - `Some(&Token)`: The next token from the token list.
    /// - `None`: When the end of the token list has been reached.
    ///
    fn next_token(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let next_token = &self.tokens[self.pos];
            self.pos += 1;
            Some(next_token)
        } else {
            None
        }
    }
}
