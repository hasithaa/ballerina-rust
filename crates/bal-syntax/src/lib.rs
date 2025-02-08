//! Ballerina Syntax Library
//! Handles lexing, tokenization, and basic syntax structures

use rowan::{GreenNode, GreenNodeBuilder, Language, SyntaxNode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BallerinaLanguage {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    // Tokens
    WHITESPACE,
    COMMENT,
    IDENTIFIER,
    INTEGER_LITERAL,
    
    // Keywords
    IMPORT_KW,
    PUBLIC_KW,
    FUNCTION_KW,
    RETURNS_KW,
    INT_KW,
    BOOLEAN_KW,
    IF_KW,
    ELSE_KW,
    WHILE_KW,
    BREAK_KW,
    CONTINUE_KW,
    RETURN_KW,
    TRUE_KW,
    FALSE_KW,
    
    // Operators
    EQ,         // ==
    NOT_EQ,     // !=
    LT,         // <
    LT_EQ,      // <=
    GT,         // >
    GT_EQ,      // >=
    PLUS,       // +
    MINUS,      // -
    STAR,       // *
    SLASH,      // /
    PERCENT,    // %
    ASSIGN,     // =
    
    // Delimiters
    L_PAREN,    // (
    R_PAREN,    // )
    L_BRACE,    // {
    R_BRACE,    // }
    COMMA,      // ,
    COLON,      // :
    SEMICOLON,  // ;
    
    // Composite nodes
    SOURCE_FILE,
    MODULE_PART,
    IMPORT_DECL,
    MODULE_DECL,
    FUNCTION_DEF,
    SIGNATURE,
    PARAM_LIST,
    PARAM,
    TYPE_DESC,
    STMT_BLOCK,
    STATEMENT,
    LOCAL_VAR_DECL_STMT,
    FUNCTION_CALL_STMT,
    ASSIGN_STMT,
    RETURN_STMT,
    IF_ELSE_STMT,
    WHILE_STMT,
    BREAK_STMT,
    CONTINUE_STMT,
    EXPRESSION,
    EQUALITY_EXPR,
    RELATIONAL_EXPR,
    ADDITIVE_EXPR,
    MULTIPLICATIVE_EXPR,
    UNARY_EXPR,
    PRIMARY_EXPR,
    FUNCTION_CALL_EXPR,
    ARG_LIST,
    QUALIFIED_IDENTIFIER,
    
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
pub mod error;

#[cfg(test)]
mod lexer_test; 