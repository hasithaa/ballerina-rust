//! Ballerina Syntax Library
//! Handles lexing, tokenization, and basic syntax structures

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
    NEWLINE,

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
    EQ,      // ==
    NOT_EQ,  // !=
    LT,      // <
    LT_EQ,   // <=
    GT,      // >
    GT_EQ,   // >=
    PLUS,    // +
    MINUS,   // -
    STAR,    // *
    SLASH,   // /
    PERCENT, // %
    ASSIGN,  // =

    // Delimiters
    L_PAREN,   // (
    R_PAREN,   // )
    L_BRACE,   // {
    R_BRACE,   // }
    COMMA,     // ,
    COLON,     // :
    SEMICOLON, // ;

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

pub mod error;
pub mod lexer;
pub mod project;
pub mod token;

#[cfg(test)]
mod lexer_test;

// Add this function to convert between token types
pub fn convert_token(token: lexer::Token) -> SyntaxKind {
    match token {
        lexer::Token::Import => SyntaxKind::IMPORT_KW,
        lexer::Token::Public => SyntaxKind::PUBLIC_KW,
        lexer::Token::Function => SyntaxKind::FUNCTION_KW,
        lexer::Token::Returns => SyntaxKind::RETURNS_KW,
        lexer::Token::Int => SyntaxKind::INT_KW,
        lexer::Token::Boolean => SyntaxKind::BOOLEAN_KW,
        lexer::Token::Identifier => SyntaxKind::IDENTIFIER,
        lexer::Token::LParen => SyntaxKind::L_PAREN,
        lexer::Token::RParen => SyntaxKind::R_PAREN,
        lexer::Token::LBrace => SyntaxKind::L_BRACE,
        lexer::Token::RBrace => SyntaxKind::R_BRACE,
        lexer::Token::Comma => SyntaxKind::COMMA,
        lexer::Token::Semicolon => SyntaxKind::SEMICOLON,
        lexer::Token::Slash => SyntaxKind::SLASH,
        lexer::Token::LineComment => SyntaxKind::COMMENT,
        lexer::Token::Newline => SyntaxKind::WHITESPACE,
        _ => SyntaxKind::ERROR,
    }
}
