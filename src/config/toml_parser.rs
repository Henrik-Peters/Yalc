use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;

use crate::config::Config;
use crate::config::toml_lexer::Lexer;
use crate::config::toml_lexer::Token;

use crate::config::toml_lexer::Value as LValue;

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

impl From<LValue> for Value {
    fn from(value: LValue) -> Self {
        match value {
            LValue::Bool(v) => Value::Bool(v),
            LValue::String(v) => Value::String(v),
            LValue::Integer(v) => Value::Integer(v),
            LValue::Float(v) => Value::Float(v),
        }
    }
}

impl From<&LValue> for Value {
    fn from(value: &LValue) -> Self {
        match value {
            LValue::Bool(v) => Value::Bool(*v),
            LValue::String(v) => Value::String(v.clone()),
            LValue::Integer(v) => Value::Integer(*v),
            LValue::Float(v) => Value::Float(*v),
        }
    }
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

    /// Retrieves the next token that is relevant for parsing
    ///
    /// This function internally calls the 'next_token' function
    /// to get the next token and will then filter out irrelevant tokens.
    ///
    fn next_significant_token(&mut self) -> Option<&Token> {
        while let Some(tok) = self.next_token() {
            match tok {
                Token::Whitespace | Token::Newline | Token::Comment(_) => continue,
                _ => break,
            }
        }

        //Get the reference of the current token
        if self.pos < self.tokens.len() {
            let cur_token = &self.tokens[self.pos];
            Some(cur_token)
        } else {
            None
        }
    }

    /// Return an error when the next token is not equal to the expected_token
    fn expect_token(&mut self, expected_token: Token) -> Result<&Token, io::Error> {
        if let Some(tok) = self.next_significant_token() {
            if *tok == expected_token {
                Ok(tok)
            } else {
                Err(io::Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "Expected next toml token: {:?}, got {:?}",
                        expected_token, tok
                    ),
                ))
            }
        } else {
            Err(io::Error::new(
                ErrorKind::UnexpectedEof,
                format!(
                    "Expected next toml token {:?}, but no token found",
                    expected_token
                ),
            ))
        }
    }

    /// Return an error when the next token is not a value token
    fn expect_value_token(&mut self) -> Result<&LValue, io::Error> {
        let next_token = self.next_significant_token();

        if let Some(Token::Value(v)) = next_token {
            Ok(v)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("Expected next toml token: Value, got {:?}", next_token),
            ))
        }
    }

    pub fn parse(&mut self) -> Result<TopLevelTable, io::Error> {
        let mut root: TopLevelTable = HashMap::new();
        let mut current_context: &mut Table = &mut root;

        while let Some(token) = self.next_significant_token() {
            match token {
                Token::Key(key) => {
                    //After a key there must an equal and value token
                    self.expect_token(Token::Equal)?;
                    let value = self.expect_value_token()?;

                    //Insert into the correct table
                    Self::insert_into_table(&mut current_context, &key, &value)?;
                }
                Token::EOF => break,
                _ => continue, //Ignore comments/whitespace
            }
        }

        Ok(root)
    }

    fn insert_into_table(table: &mut Table, key: &Key, value: &LValue) -> Result<(), io::Error> {
        //Get the corresponding entry in the map for in-place manipulation
        match table.entry(key.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(value.into());
            }
            Entry::Occupied(mut _entry) => {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    format!("Duplicate toml key: {}", key),
                ));
            }
        };

        Ok(())
    }
}
