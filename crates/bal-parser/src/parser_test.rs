#[cfg(test)]
mod tests {
    use crate::Parser;
    use bal_syntax::SyntaxKind;
    use bal_syntax::error::Span;

    fn parse(input: &str) -> String {
        let tokens = tokenize(input);
        let parser = Parser::new(Some("test.bal".to_string()), tokens);
        let node = parser.parse();
        format!("{:#?}", node)
    }

    fn tokenize(input: &str) -> Vec<(SyntaxKind, String, Span)> {
        use bal_syntax::lexer::{Lexer, Token};
        
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        
        while let Some(result) = lexer.next_token() {
            if let Ok(token_info) = result {
                // Skip whitespace and newlines
                match token_info.kind {
                    Token::Import => tokens.push((SyntaxKind::IMPORT_KW, token_info.text, token_info.span)),
                    Token::Public => tokens.push((SyntaxKind::PUBLIC_KW, token_info.text, token_info.span)),
                    Token::Function => tokens.push((SyntaxKind::FUNCTION_KW, token_info.text, token_info.span)),
                    Token::Returns => tokens.push((SyntaxKind::RETURNS_KW, token_info.text, token_info.span)),
                    Token::Int => tokens.push((SyntaxKind::INT_KW, token_info.text, token_info.span)),
                    Token::Boolean => tokens.push((SyntaxKind::BOOLEAN_KW, token_info.text, token_info.span)),
                    Token::Identifier => tokens.push((SyntaxKind::IDENTIFIER, token_info.text, token_info.span)),
                    Token::LParen => tokens.push((SyntaxKind::L_PAREN, token_info.text, token_info.span)),
                    Token::RParen => tokens.push((SyntaxKind::R_PAREN, token_info.text, token_info.span)),
                    Token::LBrace => tokens.push((SyntaxKind::L_BRACE, token_info.text, token_info.span)),
                    Token::RBrace => tokens.push((SyntaxKind::R_BRACE, token_info.text, token_info.span)),
                    Token::Comma => tokens.push((SyntaxKind::COMMA, token_info.text, token_info.span)),
                    Token::Semicolon => tokens.push((SyntaxKind::SEMICOLON, token_info.text, token_info.span)),
                    Token::Slash => tokens.push((SyntaxKind::SLASH, token_info.text, token_info.span)),
                    Token::LineComment => tokens.push((SyntaxKind::COMMENT, token_info.text, token_info.span)),
                    Token::Newline => continue, // Skip newlines
                    _ => continue, // Skip other tokens
                }
            }
        }
        tokens
    }

    #[test]
    fn test_empty_file() {
        let input = "";
        insta::assert_snapshot!(parse(input));
    }

    #[test]
    fn test_simple_import() {
        let input = "import foo/bar;";
        insta::assert_snapshot!(parse(input));
    }

    #[test]
    fn test_simple_function() {
        let input = "function add(int a, int b) returns int { }";
        insta::assert_snapshot!(parse(input));
    }

    #[test]
    fn test_public_function() {
        let input = "public function greet() { }";
        insta::assert_snapshot!(parse(input));
    }

    #[test]
    fn test_function_with_multiple_params() {
        let input = "function calc(int x, boolean flag, int y) { }";
        insta::assert_snapshot!(parse(input));
    }

    #[test]
    fn test_complete_module() {
        let input = r#"
            import ballerina/io;
            
            public function add(int a, int b) returns int {
            }
            
            function helper(boolean flag) {
            }
        "#;
        insta::assert_snapshot!(parse(input));
    }

    #[test]
    fn test_error_recovery_missing_semicolon() {
        let input = "import foo/bar";  // Missing semicolon
        insta::assert_snapshot!(parse(input));
    }

    #[test]
    fn test_error_recovery_missing_param_type() {
        let input = "function bad(a, int b) { }";  // Missing type for first param
        insta::assert_snapshot!(parse(input));
    }
} 