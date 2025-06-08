//! Module for the yalc toml lexer logic
//!
//! Provides logic for parsing toml tokens from a UTF-8 char sequence.
//! The tokens already contain some higher logic like key-value separation.
//! Note that this parser implementation does not cover all toml features.
//!

/// Strings are used to represent TOML keys
type Key = String;

/// String are used to represent TOML section titles
type SectionName = String;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// Represents a key in a key-value pair.
    /// The associated `String` is the name of the key.
    Key(Key),

    /// Represents the equal sign (`=`) separating keys and values.
    Equal,

    /// Represents the value associated with a key.
    /// The `Value` can be any of the supported TOML data types (e.g., bool, string, integer, float).
    Value(Value),

    /// Represents a comma (`,`) used in lists.
    Comma,

    /// Represents the left square bracket (`[`) marking the start of a section.
    LBracket,

    /// Represents the right square bracket (`]`) marking the end of a section.
    RBracket,

    /// Represents the left square brackets (`[[`) marking the start of an array.
    DoubleLBracket,

    /// Represents the right square brackets (`]]`) marking the end of an array.
    DoubleRBracket,

    /// Header title of a section or array enclosed by square brackets
    SectionName(SectionName),

    /// Whitespace characters like spaces, tabs, or newlines are ignored
    Whitespace,

    /// The associated `String` contains the text of the comment.
    /// Comments in TOML start with a hash symbol (`#`) and continue to the end of the line.
    Comment(String),

    /// Represents a newline character.
    Newline,

    /// Represents the end of the file (EOF) token.
    EOF,

    /// Represents an error during tokenization.
    /// The associated `String` contains the error message.
    Error(String),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    String(String),
    Integer(i64),
    Float(f64),
}

pub struct Lexer {
    /// Vector with all UTF-8 chars for the given input
    chars: Vec<char>,

    /// Index of the next char that will be processed
    pos: usize,

    /// Equal sign was consumed in current line when true
    equals_consumed: bool,

    /// Square brackets char was consumed in current line when true
    bracket_consumed: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
            equals_consumed: false,
            bracket_consumed: false,
        }
    }

    /// Retrieves the next character from the input string
    ///
    /// The 'next_char()' function returns correct Unicode characters,
    /// even if they require several bytes in UTF-8. A char in rust
    /// represents a Unicode scalar which can have different byte lengths.
    ///
    /// # Returns
    /// - `Some(char)`: The next character from the input string.
    /// - `None`: When the end of the string has been reached.
    ///
    /// # Example
    /// The input 'abc ä ö ü' will return [abc, ,ä, ,ö, ,ü] (ä is 2 bytes long in UTF-8)
    ///
    fn next_char(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    /// Similar to next_char() but pos will not be incremented
    ///
    /// This function can used to need at the next char that
    /// will be consumed by the call of next_char() function.
    ///
    /// # Returns
    /// - `Some(char)`: The next character from the input string.
    /// - `None`: When the end of the string has been reached.
    ///
    fn look_ahead_char(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            Some(c)
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Token {
        //Get the next char for whitespace check
        let next_char: Option<char> = self.next_char();
        let look_ahead_char: Option<char> = self.look_ahead_char();

        match next_char {
            None => Token::EOF,
            Some(c) => {
                //Check if the char is a whitespace
                if c.is_whitespace() {
                    //Handle line breaks which are whitespaces
                    if c == '\n' {
                        //Reset consumed chars at new line
                        self.equals_consumed = false;
                        self.bracket_consumed = false;
                        Token::Newline
                    } else {
                        Token::Whitespace
                    }
                } else {
                    //Double brackets
                    if let Some(ac) = look_ahead_char {
                        if c == '[' && ac == '[' {
                            self.bracket_consumed = true;
                            self.next_char(); //Consume the ahead char
                            return Token::DoubleLBracket;
                        }

                        if c == ']' && ac == ']' {
                            self.next_char(); //Consume the ahead char
                            return Token::DoubleRBracket;
                        }
                    }

                    //Handle Non-Whitespace chars
                    match c {
                        '=' => {
                            // Equal sign
                            self.equals_consumed = true;
                            Token::Equal
                        }
                        ',' => Token::Comma, // Comma
                        '[' => {
                            // Left bracket
                            self.bracket_consumed = true;
                            Token::LBracket
                        }
                        ']' => Token::RBracket,      // Right bracket
                        '"' => self.parse_string(),  // Handle string values
                        '#' => self.parse_comment(), // Handle comments
                        _ if c.is_alphanumeric() || c == '_' => self.parse_key_or_value(c),
                        _ => Token::Error("Unknown token".to_string()), // Handle any unexpected characters
                    }
                }
            }
        }
    }

    /// Parse a section that can be a key or a value
    fn parse_key_or_value(&mut self, first_char: char) -> Token {
        //The value can not be a string - this was handled earlier
        if self.bracket_consumed {
            self.parse_section_name(first_char)
        } else {
            //Parse non-section headers
            if !&self.equals_consumed {
                self.parse_key(first_char)
            } else {
                self.parse_value(first_char)
            }
        }
    }

    /// Parse the key token and consume all chars of the key
    fn parse_key(&mut self, first_char: char) -> Token {
        let mut key = first_char.to_string();

        while let Some(c) = self.look_ahead_char() {
            if c.is_alphanumeric() || c == '_' || c == '.' {
                //Consume the next char
                let next_char = self.next_char();

                if let Some(c) = next_char {
                    key.push(c);
                }
            } else {
                break; //End of key
            }
        }

        Token::Key(key)
    }

    /// Parse a value that is not a string value
    fn parse_value(&mut self, first_char: char) -> Token {
        let mut value_str = first_char.to_string();

        while let Some(c) = self.look_ahead_char() {
            if c.is_alphanumeric() || c == '.' || c == '_' || c == '-' {
                // Consume the next character
                let next_char = self.next_char();
                if let Some(c) = next_char {
                    value_str.push(c);
                }
            } else {
                break;
            }
        }

        //Try parsing as bool
        if let Some(bool_token) = self.try_parse_bool_value(&value_str) {
            return bool_token;
        }

        //Try parsing as integer
        if let Ok(int_val) = value_str.parse::<i64>() {
            return Token::Value(Value::Integer(int_val));
        }

        //Try parsing as float
        if let Ok(float_val) = value_str.parse::<f64>() {
            return Token::Value(Value::Float(float_val));
        }

        //If nothing matched, treat it as a error
        Token::Error("Invalid value data type".to_string())
    }

    /// Try to parse a value as boolean
    fn try_parse_bool_value(&mut self, value_str: &str) -> Option<Token> {
        match value_str {
            "true" => Some(Token::Value(Value::Bool(true))),
            "false" => Some(Token::Value(Value::Bool(false))),
            _ => None,
        }
    }

    /// Parse the section title between square brackets
    fn parse_section_name(&mut self, first_char: char) -> Token {
        let mut section_name = first_char.to_string();

        while let Some(c) = self.look_ahead_char() {
            if c.is_alphanumeric() || c == '_' || c == '.' {
                //Consume the next char
                let next_char = self.next_char();

                if let Some(c) = next_char {
                    section_name.push(c);
                }
            } else {
                break; //End of section name
            }
        }

        Token::SectionName(section_name)
    }

    /// Parse values that are identified by string quotes
    fn parse_string(&mut self) -> Token {
        let mut string_value = String::new();

        while let Some(c) = self.look_ahead_char() {
            if c == '"' {
                self.next_char(); //Consume the end of the string char
                break; //End of the string
            }

            //Consume the next char
            let next_char = self.next_char();

            if let Some(c) = next_char {
                string_value.push(c);
            }
        }

        Token::Value(Value::String(string_value))
    }

    /// Parse comment lines identified by the #-char
    fn parse_comment(&mut self) -> Token {
        let mut comment_value = String::new();

        while let Some(c) = self.look_ahead_char() {
            if c == '\n' {
                //End of comment at the newline
                break;
            }

            //Consume the next char
            let next_char = self.next_char();

            if let Some(c) = next_char {
                comment_value.push(c); //Collect comment contents
            }
        }

        Token::Comment(comment_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_key_value_string() {
        let input = r#"hello = "world""#;
        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::Key("hello".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("world".to_string())),
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_key_value_integer() {
        let input = "key = 1";
        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::Key("key".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Integer(1)),
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_key_value_boolean() {
        let input = "key = true";
        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::Key("key".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Bool(true)),
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_key_value_float() {
        let input = "key = 12.3";
        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::Key("key".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Float(12.3)),
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_multiline() {
        let input = r#"name = "test"
age = 30
"#;

        let mut lexer = Lexer::new(input);
        let tokens = vec![
            Token::Key("name".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("test".to_string())),
            Token::Newline,
            Token::Key("age".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Integer(30)),
            Token::Newline,
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_list() {
        let input = r#"name = "list-test"
file_list = [
    "apple",
    "banana",
    "cherry"
]
"#;

        let mut lexer = Lexer::new(input);
        let tokens = vec![
            Token::Key("name".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("list-test".to_string())),
            Token::Newline,
            Token::Key("file_list".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::LBracket,
            //List element [0]
            Token::Newline,
            Token::Whitespace,
            Token::Whitespace,
            Token::Whitespace,
            Token::Whitespace,
            Token::Value(Value::String("apple".to_string())),
            Token::Comma,
            //List element [1]
            Token::Newline,
            Token::Whitespace,
            Token::Whitespace,
            Token::Whitespace,
            Token::Whitespace,
            Token::Value(Value::String("banana".to_string())),
            Token::Comma,
            //List element [2]
            Token::Newline,
            Token::Whitespace,
            Token::Whitespace,
            Token::Whitespace,
            Token::Whitespace,
            Token::Value(Value::String("cherry".to_string())),
            Token::Newline,
            Token::RBracket,
            Token::Newline,
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_comment_line() {
        let input = r#"name = "comment-test"
# Text of my comment
"#;

        let mut lexer = Lexer::new(input);
        let tokens = vec![
            Token::Key("name".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("comment-test".to_string())),
            Token::Newline,
            Token::Comment(" Text of my comment".to_string()),
            Token::Newline,
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_section() {
        let input = r#"name = "section-test"
[retention]
file_size_mb = 24
"#;

        let mut lexer = Lexer::new(input);
        let tokens = vec![
            Token::Key("name".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("section-test".to_string())),
            Token::Newline,
            Token::LBracket,
            Token::SectionName("retention".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("file_size_mb".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Integer(24)),
            Token::Newline,
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_array_section() {
        let input = r#"key = "array-table-test"

[[products]]
name = "Apple"
price = 1.20

[[products]]
name = "Banana"
price = 0.80
"#;

        let mut lexer = Lexer::new(input);
        let tokens = vec![
            Token::Key("key".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("array-table-test".to_string())),
            Token::Newline,
            Token::Newline,
            //Array section [0]
            Token::DoubleLBracket,
            Token::SectionName("products".to_string()),
            Token::DoubleRBracket,
            Token::Newline,
            // Items of element [0]
            Token::Key("name".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("Apple".to_string())),
            Token::Newline,
            Token::Key("price".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Float(1.20)),
            Token::Newline,
            Token::Newline,
            //Array section [1]
            Token::DoubleLBracket,
            Token::SectionName("products".to_string()),
            Token::DoubleRBracket,
            Token::Newline,
            // Items of element [0]
            Token::Key("name".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("Banana".to_string())),
            Token::Newline,
            Token::Key("price".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Float(0.80)),
            Token::Newline,
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_section_dot_name() {
        let input = r#"name = "section-dot-test"
[retention.config]
key_amount = 123
"#;

        let mut lexer = Lexer::new(input);
        let tokens = vec![
            Token::Key("name".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("section-dot-test".to_string())),
            Token::Newline,
            Token::LBracket,
            Token::SectionName("retention.config".to_string()),
            Token::RBracket,
            Token::Newline,
            Token::Key("key_amount".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Integer(123)),
            Token::Newline,
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_simple_carriage_returns() {
        let input = "key = 5\r\nhello = \"world\"\r\n";

        let mut lexer = Lexer::new(input);
        let tokens = vec![
            Token::Key("key".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Integer(5)),
            Token::Newline,
            Token::Key("hello".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("world".to_string())),
            Token::Newline,
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_complex_combined_tokens() {
        let input = r#"# Yalc log rotation config
dry_run = false
mode = "FileSize"

keep_rotate = 7

file_list = ["apple.log", "banana.log", "cherry.log"]

[retention]
file_size_mb = 35
last_write_h = 7
"#;

        let mut lexer = Lexer::new(input);
        let tokens = vec![
            Token::Comment(" Yalc log rotation config".to_string()),
            Token::Newline,
            Token::Key("dry_run".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Bool(false)),
            Token::Newline,
            Token::Key("mode".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::String("FileSize".to_string())),
            Token::Newline,
            Token::Newline,
            Token::Key("keep_rotate".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Integer(7)),
            Token::Newline,
            Token::Newline,
            Token::Key("file_list".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::LBracket,
            //List element [0]
            Token::Value(Value::String("apple.log".to_string())),
            Token::Comma,
            //List element [1]
            Token::Whitespace,
            Token::Value(Value::String("banana.log".to_string())),
            Token::Comma,
            //List element [2]
            Token::Whitespace,
            Token::Value(Value::String("cherry.log".to_string())),
            Token::RBracket,
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
            Token::Value(Value::Integer(35)),
            Token::Newline,
            Token::Key("last_write_h".to_string()),
            Token::Whitespace,
            Token::Equal,
            Token::Whitespace,
            Token::Value(Value::Integer(7)),
            Token::Newline,
            Token::EOF,
        ];

        for expected_token in tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }
}
