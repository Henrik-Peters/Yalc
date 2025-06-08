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
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
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

    pub fn next_token(&mut self) -> Token {
        //Get the next char for whitespace check
        let next_char: Option<char> = self.next_char();
        println!("{:?}", self.chars);
        Token::EOF
    }
}
