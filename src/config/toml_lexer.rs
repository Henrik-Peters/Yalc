/// Strings are used to represents TOML keys
type Key = String;

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

    /// Whitespace characters like spaces, tabs, or newlines are ignored
    Whitespace,

    /// The associated `String` contains the text of the comment.
    /// Comments in TOML start with a hash symbol (`#`) and continue to the end of the line.
    Comment(String),

    /// Represents the end of the file (EOF) token.
    EOF,

    /// Represents a newline character.
    Newline,

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
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
            equals_consumed: false,
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

        match next_char {
            None => Token::EOF,
            Some(c) => {
                //Check if the char is a whitespace
                if c.is_whitespace() {
                    //Handle line breaks which are whitespaces
                    if c == '\n' {
                        Token::Newline
                    } else {
                        Token::Whitespace
                    }
                } else {
                    //Handle Non-Whitespace chars
                    match c {
                        '=' => Token::Equal,         // Equal sign
                        ',' => Token::Comma,         // Comma
                        '[' => Token::LBracket,      // Left bracket
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
        if !&self.equals_consumed {
            self.parse_key(first_char)
        } else {
            self.parse_value(first_char)
        }
    }

    /// Parse the key token and consume all chars of the key
    fn parse_key(&mut self, first_char: char) -> Token {
        let mut key = first_char.to_string();
        while let Some(c) = self.look_ahead_char() {
            if c.is_alphanumeric() || c == '_' {
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
        //Try to parse the value as bool
        if first_char == 't' || first_char == 'f' {
            let res_bool_token = self.try_parse_bool_value(first_char);

            if let Some(bool_token) = res_bool_token {
                return bool_token;
            }
        }

        Token::Value(Value::Integer(4))
    }

    fn try_parse_bool_value(&mut self, first_char: char) -> Option<Token> {
        Some(Token::EOF)
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
