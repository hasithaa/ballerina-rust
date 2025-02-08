//! Lexer implementation for Ballerina

use logos::Logos;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub kind: Token,
    pub span: Range<usize>,
    pub text: String,
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
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
            source: input,
        }
    }

    pub fn next_token(&mut self) -> Option<TokenInfo> {
        self.inner.next().map(|token| {
            let span = self.inner.span();
            let text = self.source[span.clone()].to_string();
            TokenInfo {
                kind: token.unwrap(),
                span,
                text,
            }
        })
    }
} 