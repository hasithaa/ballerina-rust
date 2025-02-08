//! Ballerina Syntax Library
//! Handles lexing, tokenization, and basic syntax structures

use rowan::{GreenNode, GreenNodeBuilder, Language, SyntaxNode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BallerinaLanguage {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens
    WHITESPACE,
    COMMENT,
    IDENT,
    NUMBER,
    STRING,
    
    // Keywords
    FN_KW,
    LET_KW,
    RETURN_KW,
    
    // Punctuation
    L_PAREN,
    R_PAREN,
    L_BRACE,
    R_BRACE,
    SEMICOLON,
    
    // Composite nodes
    SOURCE_FILE,
    FUNCTION_DEF,
    PARAMETER_LIST,
    
    // Special tokens
    ERROR,
    EOF,
}

impl TryFrom<u16> for SyntaxKind {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value >= (Self::EOF as u16) {
            return Err(());
        }
        Ok(unsafe { std::mem::transmute(value) })
    }
}

pub mod lexer;
pub mod token; 