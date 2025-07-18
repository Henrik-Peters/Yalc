use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;

use crate::config::Config;
use crate::config::toml_lexer::Lexer;
use crate::config::toml_lexer::SectionName;
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

    /// Represents Timestamps in RFC 3339 format
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
            LValue::DateTime(v) => Value::DateTime(v),
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
            LValue::DateTime(v) => Value::DateTime(v.clone()),
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
            if Self::token_is_significant(&tok) {
                return Some(tok);
            }
        }

        None
    }

    /// Returns true when the token is a significant token
    fn token_is_significant(tok: &Token) -> bool {
        match tok {
            Token::Whitespace | Token::Newline | Token::Comment(_) => false,
            _ => true,
        }
    }

    /// Look at the next significant token without increment the pos cursor
    fn look_ahead_significant_token(&self) -> Option<&Token> {
        let cur_pos = self.pos.borrow();
        let mut idx_look_ahead: usize = *cur_pos + 1;

        while let Some(tok) = self.tokens.get(idx_look_ahead) {
            match tok {
                tok if !Self::token_is_significant(&tok) => {
                    //Skip irrelevant tokens
                    idx_look_ahead += 1;
                }
                _ => {
                    //We found a significant token
                    return Some(tok);
                }
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

    /// Return an error when the next token is not a section name token
    fn expect_section_name_token(&self) -> Result<&SectionName, io::Error> {
        let next_token: Option<&Token> = self.next_significant_token();

        if let Some(Token::SectionName(s)) = next_token {
            Ok(s)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Expected next toml token: SectionName, got {:?}",
                    next_token
                ),
            ))
        }
    }

    /// Convert the name of a section to a vector of keys
    fn parse_section_keys(section_name: &SectionName) -> Vec<Key> {
        section_name.split('.').map(|s| s.to_string()).collect()
    }

    pub fn parse(&self) -> Result<TopLevelTable, io::Error> {
        let mut root: TopLevelTable = HashMap::new();
        let mut context: Vec<Key> = Vec::new();

        while let Some(token) = self.next_significant_token() {
            match token {
                Token::Key(key) => {
                    //After a key there must an equal and value token
                    self.expect_token(Token::Equal)?;

                    //Perform lookahead because we can have a single or list of values
                    match self.look_ahead_significant_token() {
                        None => {
                            return Err(io::Error::new(
                                ErrorKind::UnexpectedEof,
                                format!("Unexpected Eof after equal token at key: {}", key),
                            ));
                        }
                        Some(next_token) => {
                            //The value is a list when the next token is a left square bracket
                            let is_value_list: bool = *next_token == Token::LBracket;

                            if !is_value_list {
                                //Expect a single value
                                let value = self.expect_value_token()?;

                                //Insert into the correct table
                                Self::insert_into_table(&mut root, &context, &key, value.into())?;
                            } else {
                                //Expect a list of values and insert them into the table
                                self.parse_value_list(&mut root, &context, &key)?;
                            }
                        }
                    }
                }
                Token::LBracket => {
                    //We can have a left bracket of a value array (list) or a left bracket of a section name
                    //But the value of arrays is handled by the "Key"-Case above - so it must be a section name
                    let section_name = self.expect_section_name_token()?;
                    let section_keys = Self::parse_section_keys(&section_name);

                    //Apply the new context
                    context = section_keys;

                    //Expect closing bracket after the section name
                    self.expect_token(Token::RBracket)?;
                }
                Token::DoubleLBracket => {
                    //We have an array of tables. The next token must be the section name of the array
                    let section_name = self.expect_section_name_token()?;
                    let section_keys = Self::parse_section_keys(section_name);

                    //Expect closing bracket after array of tables section name
                    self.expect_token(Token::DoubleRBracket)?;

                    //Navigate to the parent table. The last key is the array's name.
                    let (array_key, parent_keys) = section_keys.split_last().ok_or_else(|| {
                        io::Error::new(
                            ErrorKind::InvalidData,
                            "Array of tables name cannot be empty",
                        )
                    })?;

                    let mut current_table = &mut root;

                    for key in parent_keys {
                        let entry = current_table
                            .entry(key.clone())
                            .or_insert_with(|| Value::Table(Table::new()));

                        if let Value::Table(table) = entry {
                            current_table = table;
                        } else {
                            return Err(io::Error::new(
                                ErrorKind::InvalidData,
                                format!("Key '{}' in path is not a table.", key),
                            ));
                        }
                    }

                    //In the parent table, find or create the array
                    let array_value = current_table
                        .entry(array_key.clone())
                        .or_insert_with(|| Value::Array(Vec::new()));

                    //The value must be an array, append a new table to it
                    if let Value::Array(array) = array_value {
                        array.push(Value::Table(Table::new()));
                    } else {
                        return Err(io::Error::new(
                            ErrorKind::InvalidData,
                            format!("Key '{}' is not an array of tables.", array_key),
                        ));
                    }

                    //Set the context for following key-value pairs
                    context = section_keys;
                }
                Token::EOF => break,
                _ => continue, //Ignore comments/whitespace
            }
        }

        Ok(root)
    }

    /// Parse a list of values and insert them into the table - assumes the next token is LBracket
    fn parse_value_list(
        &self,
        root: &mut TopLevelTable,
        context: &Vec<Key>,
        key: &Key,
    ) -> Result<(), io::Error> {
        //A value list must start with a left bracket
        self.expect_token(Token::LBracket)?;
        let mut values: Vec<Value> = Vec::new();

        while let Some(token) = self.next_significant_token() {
            match token {
                Token::Value(v) => {
                    //Convert the LValue into a value
                    values.push(v.into());
                }
                Token::Comma => {
                    //Separator for the list elements
                }
                Token::RBracket => {
                    //The list is closed
                    let list_value: Value = Value::Array(values);
                    Self::insert_into_table(root, &context, &key, list_value)?;

                    return Ok(());
                }

                _ => break,
            }
        }

        //A value list must end with with RBracket
        return Err(io::Error::new(
            ErrorKind::UnexpectedEof,
            format!("Expected RBracket token to close a value list"),
        ));
    }

    fn insert_into_table(
        root: &mut TopLevelTable,
        context: &Vec<Key>,
        key: &Key,
        value: Value,
    ) -> Result<(), io::Error> {
        let mut current_table: &mut Table = root;

        for part in context {
            //Get a mutable reference to the value at the current context key
            let entry = current_table
                .entry(part.clone())
                .or_insert_with(|| Value::Table(HashMap::new()));

            //Now, we need to get a mutable reference to the table we want to insert into.
            //This can either be the entry itself (if it's a table) or the *last*
            //element of the entry (if it's an array of tables).
            let target_table = match entry {
                Value::Table(table) => table,
                Value::Array(array) => {
                    if let Some(Value::Table(table)) = array.last_mut() {
                        table
                    } else {
                        return Err(io::Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "Cannot insert, array '{}' does not contain tables or is empty",
                                part
                            ),
                        ));
                    }
                }
                _ => {
                    return Err(io::Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Tried to insert into context key '{}' which is not a table or array of tables",
                            part
                        ),
                    ));
                }
            };
            current_table = target_table;
        }

        //Insert the final key-value pair in the target table
        match current_table.entry(key.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
            Entry::Occupied(_) => Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("Duplicate toml key: {}", key),
            )),
        }
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

    #[test]
    fn test_root_value_list() {
        let tokens = vec![
            Token::Key("keep_rotate".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(0)),
            Token::Newline,
            Token::Key("file_list".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::LBracket,
            Token::Newline,
            Token::Whitespace,
            Token::Value(LValue::Integer(1)),
            Token::Comma,
            Token::Newline,
            Token::Whitespace,
            Token::Value(LValue::Integer(2)),
            Token::Comma,
            Token::Newline,
            Token::Whitespace,
            Token::Value(LValue::Integer(3)),
            Token::Newline,
            Token::RBracket,
            Token::Newline,
            Token::EOF,
        ];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("keep_rotate".to_string(), Value::Integer(0));
        exp_table.insert(
            "file_list".to_string(),
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ]),
        );

        assert_eq!(table, exp_table);
    }

    #[test]
    fn test_single_table() {
        let tokens = vec![
            Token::Key("keep_rotate".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(12)),
            Token::Newline,
            Token::LBracket,
            Token::SectionName("retention".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("file_size_mb".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(24)),
            Token::Newline,
            Token::Key("last_write_h".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(5)),
            Token::Newline,
            Token::EOF,
        ];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        let mut retention_table: Table = HashMap::new();

        retention_table.insert("file_size_mb".to_string(), Value::Integer(24));
        retention_table.insert("last_write_h".to_string(), Value::Integer(5));

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("keep_rotate".to_string(), Value::Integer(12));
        exp_table.insert("retention".to_string(), Value::Table(retention_table));

        assert_eq!(table, exp_table);
    }

    #[test]
    fn test_mixed_tables() {
        let tokens = vec![
            Token::Key("keep_rotate".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(12)),
            Token::Newline,
            Token::LBracket,
            Token::SectionName("retention".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("file_size_mb".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(24)),
            Token::Newline,
            Token::LBracket,
            Token::SectionName("config".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("first_config".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(1)),
            Token::Newline,
            Token::Key("second_config".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(2)),
            Token::Newline,
            Token::LBracket,
            Token::SectionName("retention".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("last_write_h".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(5)),
            Token::Newline,
            Token::EOF,
        ];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        let mut retention_table: Table = HashMap::new();

        retention_table.insert("file_size_mb".to_string(), Value::Integer(24));
        retention_table.insert("last_write_h".to_string(), Value::Integer(5));

        let mut config_table: Table = HashMap::new();

        config_table.insert("first_config".to_string(), Value::Integer(1));
        config_table.insert("second_config".to_string(), Value::Integer(2));

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("keep_rotate".to_string(), Value::Integer(12));
        exp_table.insert("retention".to_string(), Value::Table(retention_table));
        exp_table.insert("config".to_string(), Value::Table(config_table));

        assert_eq!(table, exp_table);
    }

    #[test]
    fn test_sub_tables() {
        let tokens = vec![
            Token::Key("keep_rotate".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(12)),
            Token::Newline,
            Token::LBracket,
            Token::SectionName("servers".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::LBracket,
            Token::SectionName("servers.alpha".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("ip".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(1)),
            Token::Newline,
            Token::LBracket,
            Token::SectionName("servers.beta".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("ip".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(2)),
            Token::Newline,
            Token::EOF,
        ];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        let mut servers_alpha_table: Table = HashMap::new();
        servers_alpha_table.insert("ip".to_string(), Value::Integer(1));

        let mut servers_beta_table: Table = HashMap::new();
        servers_beta_table.insert("ip".to_string(), Value::Integer(2));

        let mut servers_table: Table = HashMap::new();
        servers_table.insert("alpha".to_string(), Value::Table(servers_alpha_table));
        servers_table.insert("beta".to_string(), Value::Table(servers_beta_table));

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("keep_rotate".to_string(), Value::Integer(12));
        exp_table.insert("servers".to_string(), Value::Table(servers_table));

        assert_eq!(table, exp_table);
    }

    #[test]
    fn test_sub_table_array_value() {
        let tokens = vec![
            Token::Key("keep_rotate".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(10)),
            Token::Newline,
            Token::Newline,
            Token::LBracket,
            Token::SectionName("retention".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("file_size_mb".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(30)),
            Token::Newline,
            Token::Key("colors".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::LBracket,
            Token::Value(LValue::String("red".to_string())),
            Token::Comma,
            Token::Whitespace,
            Token::Value(LValue::String("green".to_string())),
            Token::Comma,
            Token::Whitespace,
            Token::Value(LValue::String("blue".to_string())),
            Token::RBracket,
            Token::Newline,
            Token::Newline,
            Token::Key("enable_flags".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::LBracket,
            Token::Newline,
            Token::Whitespace,
            Token::Value(LValue::Bool(true)),
            Token::Comma,
            Token::Newline,
            Token::Whitespace,
            Token::Value(LValue::Bool(true)),
            Token::Comma,
            Token::Newline,
            Token::Whitespace,
            Token::Value(LValue::Bool(false)),
            Token::Comma,
            Token::Newline,
            Token::Whitespace,
            Token::Value(LValue::Bool(true)),
            Token::Newline,
            Token::RBracket,
            Token::Newline,
            Token::Key("final_key".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(50)),
            Token::Newline,
            Token::EOF,
        ];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("keep_rotate".to_string(), Value::Integer(10));

        let mut retention_table: Table = HashMap::new();
        retention_table.insert("file_size_mb".to_string(), Value::Integer(30));
        retention_table.insert(
            "colors".to_string(),
            Value::Array(vec![
                Value::String("red".to_string()),
                Value::String("green".to_string()),
                Value::String("blue".to_string()),
            ]),
        );

        retention_table.insert(
            "enable_flags".to_string(),
            Value::Array(vec![
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(true),
            ]),
        );

        retention_table.insert("final_key".to_string(), Value::Integer(50));

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("keep_rotate".to_string(), Value::Integer(10));
        exp_table.insert("retention".to_string(), Value::Table(retention_table));

        assert_eq!(table, exp_table);
    }

    #[test]
    fn test_array_of_tables() {
        let tokens = vec![
            Token::Key("keep_rotate".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(21)),
            Token::Newline,
            Token::DoubleLBracket,
            Token::SectionName("users".to_string()),
            Token::DoubleRBracket,
            Token::Newline,
            Token::Key("age".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(1)),
            Token::Newline,
            Token::Newline,
            Token::DoubleLBracket,
            Token::SectionName("users".to_string()),
            Token::DoubleRBracket,
            Token::Newline,
            Token::Key("age".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(2)),
            Token::Newline,
            Token::Newline,
            Token::DoubleLBracket,
            Token::SectionName("users".to_string()),
            Token::DoubleRBracket,
            Token::Newline,
            Token::Key("age".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(LValue::Integer(3)),
            Token::Newline,
            Token::EOF,
        ];

        let parser = Parser::new(tokens);
        let table: TopLevelTable = parser.parse().unwrap();

        let mut exp_table: TopLevelTable = HashMap::new();
        exp_table.insert("keep_rotate".to_string(), Value::Integer(21));

        let mut table_0: Table = HashMap::new();
        table_0.insert("age".to_string(), Value::Integer(1));

        let mut table_1: Table = HashMap::new();
        table_1.insert("age".to_string(), Value::Integer(2));

        let mut table_2: Table = HashMap::new();
        table_2.insert("age".to_string(), Value::Integer(3));

        exp_table.insert(
            "users".to_string(),
            Value::Array(vec![
                Value::Table(table_0),
                Value::Table(table_1),
                Value::Table(table_2),
            ]),
        );

        assert_eq!(table, exp_table);
    }
}
