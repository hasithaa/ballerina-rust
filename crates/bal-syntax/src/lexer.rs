//! Lexer implementation for Ballerina

use logos::Logos;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,    // 1-based line number
    pub column: usize,  // 0-based column
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub kind: Token,
    pub text: String,
    pub span: Span,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]  // Ignore whitespace
pub enum Token {
    // Keywords
    #[token("import")]
    Import,
    
    #[token("public")]
    Public,
    
    #[token("function")]
    Function,
    
    #[token("returns")]
    Returns,
    
    #[token("int")]
    Int,
    
    #[token("boolean")]
    Boolean,
    
    #[token("if")]
    If,
    
    #[token("else")]
    Else,
    
    #[token("while")]
    While,
    
    #[token("break")]
    Break,
    
    #[token("continue")]
    Continue,
    
    #[token("return")]
    Return,
    
    #[token("true")]
    True,
    
    #[token("false")]
    False,
    
    // Operators
    #[token("==")]
    Eq,
    
    #[token("!=")]
    NotEq,
    
    #[token("<")]
    Lt,
    
    #[token("<=")]
    LtEq,
    
    #[token(">")]
    Gt,
    
    #[token(">=")]
    GtEq,
    
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Star,
    
    #[token("/")]
    Slash,
    
    #[token("%")]
    Percent,
    
    #[token("=")]
    Assign,
    
    // Delimiters
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,
    
    #[token(",")]
    Comma,
    
    #[token(":")]
    Colon,
    
    #[token(";")]
    Semicolon,
    
    // Identifiers and literals
    #[regex("[A-Za-z][A-Za-z0-9_]*")]
    Identifier,
    
    #[regex("0|[1-9][0-9]*")]
    IntegerLiteral,

    // Comments
    #[regex("//[^\n]*")]
    LineComment,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
    source: &'a str,
    line_starts: Vec<usize>, // Cache line start positions
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        // Calculate line starts once during initialization
        let line_starts: Vec<_> = std::iter::once(0)
            .chain(input.match_indices('\n')
                .map(|(i, _)| i + 1))
            .collect();
        
        Self {
            inner: Token::lexer(input),
            source: input,
            line_starts,
        }
    }

    fn get_position(&self, offset: usize) -> (usize, usize) {
        match self.line_starts.binary_search(&offset) {
            Ok(line) => (line + 1, 0),
            Err(line) => (line, offset - self.line_starts[line - 1]),
        }
    }

    pub fn next_token(&mut self) -> Option<TokenInfo> {
        self.inner.next().map(|token| {
            let range = self.inner.span();
            let (line, column) = self.get_position(range.start);
            TokenInfo {
                kind: token.unwrap(),
                text: self.source[range.clone()].to_string(),
                span: Span {
                    start: range.start,
                    end: range.end,
                    line,
                    column,
                },
            }
        })
    }
} 