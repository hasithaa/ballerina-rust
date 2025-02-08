//! CLI command implementations

use std::fs;
use std::path::Path;
use bal_parser::Parser;
use bal_syntax::lexer::Lexer;
use bal_syntax::SyntaxKind;


pub fn build(input: &str) -> Result<(), String> {
    let path = Path::new(input);
    if !path.exists() {
        return Err(format!("File not found: {}", input));
    }
    if path.extension().and_then(|ext| ext.to_str()) != Some("bal") {
        return Err(format!("Not a Ballerina file: {}", input));
    }

    let source = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Tokenize with error handling
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source);
    let had_errors = false;
    
    while let Some(result) = lexer.next_token() {
        if let Ok(token_info) = result {
            // Skip only whitespace and newlines
            if !matches!(token_info.kind, bal_syntax::lexer::Token::Newline) {
                let kind = convert_token(token_info.kind);
                // Store token with its span information
                tokens.push((kind, token_info.text, token_info.span));
            }
        }
    }

    if had_errors {
        return Err("Failed to tokenize input".to_string());
    }

    let file_name = path.to_str().map(String::from);
    let parser = Parser::new(file_name, tokens);
    match parser.parse() {
        Ok(parse_tree) => {
            println!("Successfully parsed: {}", input);
            println!("Parse tree:\n{:#?}", parse_tree);
            Ok(())
        }
        Err(e) => {
            Err(format!("Parser error: {}", e))
        }
    }
}

fn convert_token(token: bal_syntax::lexer::Token) -> SyntaxKind {
    match token {
        bal_syntax::lexer::Token::Import => SyntaxKind::IMPORT_KW,
        bal_syntax::lexer::Token::Public => SyntaxKind::PUBLIC_KW,
        bal_syntax::lexer::Token::Function => SyntaxKind::FUNCTION_KW,
        bal_syntax::lexer::Token::Returns => SyntaxKind::RETURNS_KW,
        bal_syntax::lexer::Token::Int => SyntaxKind::INT_KW,
        bal_syntax::lexer::Token::Boolean => SyntaxKind::BOOLEAN_KW,
        bal_syntax::lexer::Token::Identifier => SyntaxKind::IDENTIFIER,
        bal_syntax::lexer::Token::LParen => SyntaxKind::L_PAREN,
        bal_syntax::lexer::Token::RParen => SyntaxKind::R_PAREN,
        bal_syntax::lexer::Token::LBrace => SyntaxKind::L_BRACE,
        bal_syntax::lexer::Token::RBrace => SyntaxKind::R_BRACE,
        bal_syntax::lexer::Token::Comma => SyntaxKind::COMMA,
        bal_syntax::lexer::Token::Semicolon => SyntaxKind::SEMICOLON,
        bal_syntax::lexer::Token::Slash => SyntaxKind::SLASH,
        bal_syntax::lexer::Token::LineComment => SyntaxKind::COMMENT,
        bal_syntax::lexer::Token::Newline => SyntaxKind::WHITESPACE,
        _ => SyntaxKind::ERROR,
    }
} 