#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token, TokenInfo};

    fn tokenize(input: &str) -> Vec<TokenInfo> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(result) = lexer.next_token() {
            if let Ok(token) = result {
                // Skip newlines
                if !matches!(token.kind, Token::Newline) {
                    tokens.push(token);
                }
            }
        }
        tokens
    }

    #[test]
    fn test_keywords() {
        let input = "import public function returns int boolean if else while break continue return true false";
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Import,
                Token::Public,
                Token::Function,
                Token::Returns,
                Token::Int,
                Token::Boolean,
                Token::If,
                Token::Else,
                Token::While,
                Token::Break,
                Token::Continue,
                Token::Return,
                Token::True,
                Token::False,
            ]
        );
    }

    #[test]
    fn test_operators() {
        let input = "== != < <= > >= + - * / % =";
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Eq,
                Token::NotEq,
                Token::Lt,
                Token::LtEq,
                Token::Gt,
                Token::GtEq,
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Percent,
                Token::Assign,
            ]
        );
    }

    #[test]
    fn test_delimiters() {
        let input = "( ) { } , : ;";
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::RBrace,
                Token::Comma,
                Token::Colon,
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let input = "foo bar123 baz_qux";
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![Token::Identifier, Token::Identifier, Token::Identifier,]
        );
    }

    #[test]
    fn test_integer_literals() {
        let input = "0 42 123456789";
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::IntegerLiteral,
                Token::IntegerLiteral,
                Token::IntegerLiteral,
            ]
        );
    }

    #[test]
    fn test_comments() {
        let input = "foo // this is a comment\nbar baz";
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier,
                Token::LineComment,
                Token::Identifier,
                Token::Identifier,
            ]
        );
    }

    #[test]
    fn test_comment_spans() {
        let input = "foo // comment";
        let token = tokenize(input)
            .into_iter()
            .find(|t| matches!(t.kind, Token::LineComment))
            .unwrap();
        assert_eq!(token.text, "// comment");
        assert_eq!((token.span.start, token.span.end), (4, 14));
    }

    #[test]
    fn test_complex_function() {
        let input = r#"
            public function add(int a, int b) returns int {
                return a + b;
            }
        "#;
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Public,
                Token::Function,
                Token::Identifier,
                Token::LParen,
                Token::Int,
                Token::Identifier,
                Token::Comma,
                Token::Int,
                Token::Identifier,
                Token::RParen,
                Token::Returns,
                Token::Int,
                Token::LBrace,
                Token::Return,
                Token::Identifier,
                Token::Plus,
                Token::Identifier,
                Token::Semicolon,
                Token::RBrace,
            ]
        );
    }

    #[test]
    fn test_import_statement() {
        let input = "import foo/bar;";
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Import,
                Token::Identifier,
                Token::Slash,
                Token::Identifier,
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_if_else_statement() {
        let input = r#"
            if (x < 10) {
                return true;
            } else {
                return false;
            }
        "#;
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::LParen,
                Token::Identifier,
                Token::Lt,
                Token::IntegerLiteral,
                Token::RParen,
                Token::LBrace,
                Token::Return,
                Token::True,
                Token::Semicolon,
                Token::RBrace,
                Token::Else,
                Token::LBrace,
                Token::Return,
                Token::False,
                Token::Semicolon,
                Token::RBrace,
            ]
        );
    }

    #[test]
    fn test_while_loop() {
        let input = r#"
            while (i < 5) {
                i = i + 1;
                if (i == 3) {
                    continue;
                }
            }
        "#;
        let tokens: Vec<_> = tokenize(input).into_iter().map(|t| t.kind).collect();
        assert_eq!(
            tokens,
            vec![
                Token::While,
                Token::LParen,
                Token::Identifier,
                Token::Lt,
                Token::IntegerLiteral,
                Token::RParen,
                Token::LBrace,
                Token::Identifier,
                Token::Assign,
                Token::Identifier,
                Token::Plus,
                Token::IntegerLiteral,
                Token::Semicolon,
                Token::If,
                Token::LParen,
                Token::Identifier,
                Token::Eq,
                Token::IntegerLiteral,
                Token::RParen,
                Token::LBrace,
                Token::Continue,
                Token::Semicolon,
                Token::RBrace,
                Token::RBrace,
            ]
        );
    }

    #[test]
    fn test_token_position() {
        let input = "foo\nbar // comment\nbaz";
        let tokens = tokenize(input);

        assert_eq!(tokens[0].span.line, 1);
        assert_eq!(tokens[0].span.column, 0);

        assert_eq!(tokens[1].span.line, 2);
        assert_eq!(tokens[1].span.column, 0);

        assert_eq!(tokens[2].span.line, 2);
        assert_eq!(tokens[2].span.column, 4);
    }
}
