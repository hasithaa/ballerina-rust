//! CLI command implementations

use std::fs;
use std::path::{Path, PathBuf};
use bal_parser::Parser;
use bal_syntax::lexer::Lexer;
use bal_syntax::SyntaxKind;
use bal_syntax::project::Project;


fn is_bal_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| ext == "bal")
}

pub fn build(input: Option<PathBuf>) -> Result<(), String> {
    match input {
        Some(path) => {
            if path.is_dir() {
                build_project_from_path(&path)
            } else {
                build_single_file(&path)
            }
        }
        None => build_project_from_path(&std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?)
    }
}

fn build_single_file(path: &Path) -> Result<(), String> {
    // Validate file exists
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    // Validate file extension before checking project
    if !is_bal_file(path) {
        return Err(format!("Not a Ballerina file: {}", path.display()));
    }

    // Check all parent directories for Ballerina.toml
    let mut current_dir = path.parent();
    while let Some(dir) = current_dir {
        if let Some(toml_path) = bal_syntax::project::find_ballerina_toml(dir) {
            return Err(format!(
                "File {} is in a Ballerina project directory (found {} in {}). Use 'bal build' in the project directory.",
                path.display(),
                toml_path.file_name().unwrap_or_default().to_string_lossy(),
                dir.display()
            ));
        }
        current_dir = dir.parent();
    }

    parse_and_build_file(path)
}

fn build_project_from_path(project_path: &Path) -> Result<(), String> {
    let project = Project::load(project_path)
        .map_err(|e| format!("Failed to load project: {}", e))?;

    println!("Building Ballerina project: {}/{} v{}", 
        project.package.info.org,
        project.package.info.name, 
        project.package.info.version);
    println!("Project directory: {}", project.root_dir.display());

    let mut project_had_errors = false;
    
    // Build all source files, continuing after errors
    for file in &project.source_files {
        println!("\nBuilding file: {}", file.display());
        if let Err(e) = parse_and_build_file(file) {
            eprintln!("Error in {}: {}", file.display(), e);
            project_had_errors = true;
            // Continue with next file
        }
    }

    if project_had_errors {
        Err("Project build completed with errors".to_string())
    } else {
        Ok(())
    }
}

fn parse_and_build_file(path: &Path) -> Result<(), String> {
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Tokenize with error handling
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source);
    let mut had_errors = false;
    
    while let Some(result) = lexer.next_token() {
        match result {
            Ok(token_info) => {
                if !matches!(token_info.kind, bal_syntax::lexer::Token::Newline) {
                    let kind = convert_token(token_info.kind);
                    tokens.push((kind, token_info.text, token_info.span));
                }
            }
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                had_errors = true;
                // Continue lexing despite error
            }
        }
    }

    let file_name = path.to_str().map(String::from);
    let parser = Parser::new(file_name, tokens);
    match parser.parse() {
        Ok(parse_tree) => {
            println!("Parse tree:\n{:#?}", parse_tree);
            if had_errors {
                Err("Completed with errors".to_string())
            } else {
                println!("Successfully parsed: {}", path.display());
                Ok(())
            }
        }
        Err(e) => {
            eprintln!("Parser error: {}", e);
            // Continue with other files if in a project
            Err("Parser errors encountered".to_string())
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