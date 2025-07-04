use std::cell::RefCell;
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
    let parser = Parser::new(tokens);
    let table: TopLevelTable = parser.parse()?;

    println!("{:?}", table);

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
    pos: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            pos: RefCell::new(0),
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
    fn next_token(&self) -> Option<&Token> {
        let mut pos = self.pos.borrow_mut();

        if *pos < self.tokens.len() {
            let next_token = &self.tokens[*pos];
            *pos += 1;
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
    fn next_significant_token(&self) -> Option<&Token> {
        while let Some(tok) = self.next_token() {
            match tok {
                Token::Whitespace | Token::Newline | Token::Comment(_) => continue,
                _ => return Some(tok),
            }
        }

        None
    }

    /// Return an error when the next token is not equal to the expected_token
    fn expect_token(&self, expected_token: Token) -> Result<&Token, io::Error> {
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
    fn expect_value_token(&self) -> Result<&LValue, io::Error> {
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

    pub fn parse(&self) -> Result<TopLevelTable, io::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tokens() {
        let tokens = vec![Token::Whitespace, Token::EOF];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        assert!(table.is_empty());
    }

    #[test]
    fn test_root_single_key_value() {
        let tokens = vec![
            Token::Key("hello".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::String("world".to_string())),
            Token::EOF,
        ];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("hello".to_string(), Value::String("world".to_string()));

        assert_eq!(table, exp_table);
    }

    #[test]
    fn test_root_multi_key_value() {
        let tokens = vec![
            Token::Key("keep_rotate".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Whitespace,
            Token::Value(LValue::Integer(3)),
            Token::Newline,
            Token::Key("dry_run".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Bool(true)),
            Token::Newline,
            Token::EOF,
        ];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("keep_rotate".to_string(), Value::Integer(3));
        exp_table.insert("dry_run".to_string(), Value::Bool(true));

        assert_eq!(table, exp_table);
    }
}
