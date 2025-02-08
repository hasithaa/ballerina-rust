#[cfg(test)]
mod tests {
    use crate::Parser;
    use bal_syntax::SyntaxKind;

    fn parse(input: &str) -> String {
        let tokens = tokenize(input);
        let parser = Parser::new(tokens);
        let node = parser.parse();
        format!("{:#?}", node)
    }

    fn tokenize(input: &str) -> Vec<(SyntaxKind, String)> {
        use bal_syntax::lexer::{Lexer, Token};
        
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        
        while let Some(token_info) = lexer.next_token() {
            let kind = match token_info.kind {
                Token::Import => SyntaxKind::IMPORT_KW,
                Token::Public => SyntaxKind::PUBLIC_KW,
                Token::Function => SyntaxKind::FUNCTION_KW,
                Token::Returns => SyntaxKind::RETURNS_KW,
                Token::Int => SyntaxKind::INT_KW,
                Token::Boolean => SyntaxKind::BOOLEAN_KW,
                Token::Identifier => SyntaxKind::IDENTIFIER,
                Token::LParen => SyntaxKind::L_PAREN,
                Token::RParen => SyntaxKind::R_PAREN,
                Token::LBrace => SyntaxKind::L_BRACE,
                Token::RBrace => SyntaxKind::R_BRACE,
                Token::Comma => SyntaxKind::COMMA,
                Token::Semicolon => SyntaxKind::SEMICOLON,
                Token::Slash => SyntaxKind::SLASH,
                _ => continue, // Skip other tokens for now
            };
            tokens.push((kind, token_info.text));
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
            import math/basic;
            
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