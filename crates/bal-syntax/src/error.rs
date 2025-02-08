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
    pub line_content: Option<String>,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidCharacter(c, pos) => {
                writeln!(f, "error: invalid character")?;
                writeln!(f, "  |")?;
                writeln!(f, "  | found invalid character '{}' at position {}", c, pos)
            }
            LexerError::UnterminatedString(pos) => {
                writeln!(f, "error: unterminated string")?;
                writeln!(f, "  |")?;
                writeln!(f, "  | string starting at position {} is not terminated", pos)
            }
            LexerError::UnexpectedEof => {
                writeln!(f, "error: unexpected end of file")?;
                writeln!(f, "  |")?;
                writeln!(f, "  | the file ends unexpectedly")
            }
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken { expected, found, span } => {
                let file = span.file.as_deref().unwrap_or("unknown");
                writeln!(f, "error: unexpected token")?;
                writeln!(f, " --> {}:{}:{}", file, span.line, span.column + 1)?;
                writeln!(f, "  |")?;
                writeln!(f, "{} | {}", span.line, get_line_content(file, span.line).unwrap_or_default())?;
                writeln!(f, "  | {}{} expected one of {:?}, found '{}'", 
                    " ".repeat(span.column),
                    "^".repeat(found.len()),
                    expected,
                    found
                )
            }
            ParserError::MissingToken { expected, after, span } => {
                let file = span.file.as_deref().unwrap_or("unknown");
                writeln!(f, "error: missing token")?;
                writeln!(f, " --> {}:{}:{}", file, span.line, span.column + 1)?;
                writeln!(f, "  |")?;
                writeln!(f, "{} | {}", span.line, get_line_content(file, span.line).unwrap_or_default())?;
                writeln!(f, "  | {}{} missing '{}' after '{}'",
                    " ".repeat(span.column),
                    "^",
                    expected,
                    after
                )
            }
        }
    }
}

impl std::error::Error for LexerError {}
impl std::error::Error for ParserError {}

// Helper function to get the content of a specific line from a file
fn get_line_content(file: &str, line_number: usize) -> std::io::Result<String> {
    use std::io::{BufRead, BufReader};
    use std::fs::File;

    let file = File::open(file)?;
    let reader = BufReader::new(file);
    
    for (index, line) in reader.lines().enumerate() {
        if index + 1 == line_number {
            return Ok(line?);
        }
    }
    Ok(String::new())
} 