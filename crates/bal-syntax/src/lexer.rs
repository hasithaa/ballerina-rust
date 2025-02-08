//! Lexer implementation for Ballerina

use logos::Logos;
use crate::SyntaxKind;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]  // Ignore whitespace
pub enum Token {
    #[token("function")]
    Function,
    
    #[token("return")]
    Return,
    
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    
    #[regex(r#""[^"]*""#)]
    StringLiteral,
    
    #[regex(r"\d+")]
    Number,
    
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,
    
    #[token(";")]
    Semicolon,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }
} 