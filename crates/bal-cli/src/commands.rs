//! CLI command implementations

use std::fs;
use std::path::Path;
use bal_parser::Parser;
use bal_syntax::lexer::Lexer;
use bal_syntax::SyntaxKind;

pub fn compile(input: &str, output: Option<&str>) -> Result<(), String> {
    // TODO: Implement compilation
    Ok(())
}

pub fn build(input: &str) -> Result<(), String> {
    // Check if file exists and has .bal extension
    let path = Path::new(input);
    if !path.exists() {
        return Err(format!("File not found: {}", input));
    }
    if path.extension().and_then(|ext| ext.to_str()) != Some("bal") {
        return Err(format!("Not a Ballerina file: {}", input));
    }

    // Read file contents
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Tokenize
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source);
    
    while let Some(token_info) = lexer.next_token() {
        let kind = match token_info.kind {
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
            _ => continue, // Skip other tokens for now
        };
        tokens.push((kind, token_info.text));
    }

    // Parse
    let parser = Parser::new(tokens);
    let parse_tree = parser.parse();

    println!("Successfully parsed: {}", input);
    println!("Parse tree:\n{:#?}", parse_tree);

    Ok(())
} 