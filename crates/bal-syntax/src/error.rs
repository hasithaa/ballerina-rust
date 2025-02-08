use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    InvalidCharacter(char, usize),
    UnterminatedString(usize),
    UnexpectedEof,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken {
        expected: Vec<String>,
        found: String,
        span: Span,
    },
    MissingToken {
        expected: String,
        after: String,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub file: Option<String>,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidCharacter(c, pos) => 
                write!(f, "Invalid character '{}' at position {}", c, pos),
            LexerError::UnterminatedString(pos) => 
                write!(f, "Unterminated string starting at position {}", pos),
            LexerError::UnexpectedEof => 
                write!(f, "Unexpected end of file"),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken { expected, found, span } => {
                write!(f, "{}: Expected one of {:?} but found '{}' at {}:{}:{}", 
                    span.file.as_deref().unwrap_or("unknown"), 
                    expected, found, span.line, span.column,
                    span.start)
            }
            ParserError::MissingToken { expected, after, span } => {
                write!(f, "Missing {} after {} at {}:{}", 
                    expected, after, span.line, span.column)
            }
        }
    }
}

impl std::error::Error for LexerError {}
impl std::error::Error for ParserError {} 