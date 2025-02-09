//! Grammar rules for Ballerina parser

use super::Parser;
use bal_syntax::error::{ParserError, Span};
use bal_syntax::{BallerinaLanguage, SyntaxKind};
use rowan::Language;

impl Parser {
    pub(crate) fn parse_module_part(&mut self) -> std::result::Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::MODULE_PART));

        // Parse imports
        while self.at(SyntaxKind::IMPORT_KW) {
            self.parse_import_decl()?;
        }

        // Parse function declarations
        while !self.at_end() && self.at_function_start() {
            self.parse_module_decl()?;
        }

        self.builder.finish_node();
        Ok(())
    }

    fn parse_import_decl(&mut self) -> Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::IMPORT_DECL));

        self.expect(SyntaxKind::IMPORT_KW)?;
        self.expect(SyntaxKind::IDENTIFIER)?;
        self.expect(SyntaxKind::SLASH)?;
        self.expect(SyntaxKind::IDENTIFIER)?;
        self.expect(SyntaxKind::SEMICOLON)
            .map_err(|_| ParserError::MissingToken {
                expected: "semicolon".to_string(),
                after: "import statement".to_string(),
                span: self.current_span(),
            })?;

        self.builder.finish_node();
        Ok(())
    }

    fn parse_module_decl(&mut self) -> Result<(), ParserError> {
        // Currently only function definitions are supported
        self.parse_function_def()?;
        Ok(())
    }

    fn parse_function_def(&mut self) -> Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::FUNCTION_DEF));

        // Optional public modifier
        if self.at(SyntaxKind::PUBLIC_KW) {
            self.bump()?;
        }

        self.expect(SyntaxKind::FUNCTION_KW)?;
        self.expect(SyntaxKind::IDENTIFIER)?;
        self.parse_signature()?;
        self.parse_stmt_block()?;

        self.builder.finish_node();
        Ok(())
    }

    fn parse_signature(&mut self) -> Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::SIGNATURE));

        self.expect(SyntaxKind::L_PAREN)?;
        if !self.at(SyntaxKind::R_PAREN) {
            self.parse_param_list()?;
        }
        self.expect(SyntaxKind::R_PAREN)?;

        if self.at(SyntaxKind::RETURNS_KW) {
            self.bump()?;
            self.parse_type_desc()?;
        }

        self.builder.finish_node();
        Ok(())
    }

    fn parse_param_list(&mut self) -> Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::PARAM_LIST));

        loop {
            self.parse_param()?;
            if !self.at(SyntaxKind::COMMA) {
                break;
            }
            self.bump()?; // Consume comma
        }

        self.builder.finish_node();
        Ok(())
    }

    fn parse_param(&mut self) -> Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::PARAM));
        self.parse_type_desc()?;
        self.expect(SyntaxKind::IDENTIFIER)?;
        self.builder.finish_node();
        Ok(())
    }

    fn parse_type_desc(&mut self) -> Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::TYPE_DESC));
        self.expect_one_of(&[SyntaxKind::INT_KW, SyntaxKind::BOOLEAN_KW])?;
        self.builder.finish_node();
        Ok(())
    }

    fn parse_stmt_block(&mut self) -> Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::STMT_BLOCK));

        self.expect(SyntaxKind::L_BRACE)?;

        while !self.at(SyntaxKind::R_BRACE) && !self.at_end() {
            match self.parse_statement() {
                Ok(_) => {}
                Err(e) => {
                    // Report error but continue parsing
                    eprintln!("Error in statement: {}", e);
                    self.synchronize();
                }
            }
        }

        self.expect(SyntaxKind::R_BRACE)?;
        self.builder.finish_node();
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<(), ParserError> {
        self.builder
            .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::STATEMENT));

        // For now, just expect a semicolon-terminated statement
        while !self.at(SyntaxKind::SEMICOLON) && !self.at_end() {
            self.bump()?;
        }
        self.expect(SyntaxKind::SEMICOLON)?;

        self.builder.finish_node();
        Ok(())
    }

    // Helper methods
    fn at(&self, kind: SyntaxKind) -> bool {
        let pos = self.next_non_trivia();
        pos < self.tokens.len() && self.tokens[pos].0 == kind
    }

    fn next_non_trivia(&self) -> usize {
        let mut pos = self.cursor;
        while pos < self.tokens.len() {
            let kind = self.tokens[pos].0;
            if !matches!(kind, SyntaxKind::WHITESPACE | SyntaxKind::COMMENT) {
                break;
            }
            pos += 1;
        }
        pos
    }

    // fn skip_trivia(&mut self) {
    //     while let Some(kind) = self.peek_kind() {
    //         if matches!(kind, SyntaxKind::WHITESPACE | SyntaxKind::COMMENT) {
    //             self.cursor += 1;
    //         } else {
    //             break;
    //         }
    //     }
    // }

    fn at_end(&self) -> bool {
        self.peek_kind().is_none()
    }

    fn peek_kind(&self) -> Option<SyntaxKind> {
        let pos = self.next_non_trivia();
        if pos < self.tokens.len() {
            Some(self.tokens[pos].0)
        } else {
            None
        }
    }

    fn bump(&mut self) -> Result<(), ParserError> {
        if self.cursor < self.tokens.len() {
            let (kind, text, _) = self.tokens[self.cursor].clone();
            self.builder
                .token(BallerinaLanguage::kind_to_raw(kind), &text);
            self.cursor += 1;
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken {
                expected: vec!["token".to_string()],
                found: "end of file".to_string(),
                span: self.current_span(),
            })
        }
    }

    fn expect(&mut self, kind: SyntaxKind) -> Result<(), ParserError> {
        if self.at(kind) {
            self.bump()?;
            Ok(())
        } else {
            let err = ParserError::UnexpectedToken {
                expected: vec![format!("{:?}", kind)],
                found: format!("{:?}", self.peek_kind().unwrap_or(SyntaxKind::EOF)),
                span: self.current_span(),
            };

            // Mark error in syntax tree
            self.builder
                .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::ERROR));
            self.builder.finish_node();

            // Recover at next statement
            self.synchronize();

            Err(err)
        }
    }

    fn expect_one_of(&mut self, kinds: &[SyntaxKind]) -> Result<(), ParserError> {
        if let Some(current) = self.peek_kind() {
            if kinds.contains(&current) {
                self.bump()?;
                Ok(())
            } else {
                let err = ParserError::UnexpectedToken {
                    expected: kinds.iter().map(|&k| format!("{:?}", k)).collect(),
                    found: format!("{:?}", current),
                    span: self.current_span(),
                };

                self.builder
                    .start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::ERROR));
                self.builder.finish_node();

                self.recover_until(&[SyntaxKind::SEMICOLON, SyntaxKind::R_BRACE]);

                Err(err)
            }
        } else {
            Err(ParserError::UnexpectedToken {
                expected: kinds.iter().map(|&k| format!("{:?}", k)).collect(),
                found: "end of file".to_string(),
                span: self.current_span(),
            })
        }
    }

    fn current_span(&self) -> Span {
        let pos = self.next_non_trivia();
        if pos < self.tokens.len() {
            let (_, _, span) = &self.tokens[pos];
            Span {
                file: self.file.clone(),
                ..span.clone()
            }
        } else {
            // Create EOF span
            Span {
                file: self.file.clone(),
                start: self.tokens.len(),
                end: self.tokens.len(),
                line: self.tokens.last().map(|(_, _, s)| s.line).unwrap_or(1),
                column: self
                    .tokens
                    .last()
                    .map(|(_, _, s)| s.column + 1)
                    .unwrap_or(0),
                line_content: None,
            }
        }
    }

    fn at_function_start(&self) -> bool {
        self.at(SyntaxKind::FUNCTION_KW) || self.at(SyntaxKind::PUBLIC_KW)
    }

    // Add this helper method for error recovery
    fn recover_until(&mut self, sync_tokens: &[SyntaxKind]) {
        while let Some(kind) = self.peek_kind() {
            if sync_tokens.contains(&kind) || kind == SyntaxKind::NEWLINE {
                break;
            }
            // Skip the current token
            let _ = self.bump();
        }
    }

    // Add these helper methods
    fn skip_until_sync_point(&mut self) {
        while let Some(kind) = self.peek_kind() {
            if self.is_sync_point(kind) {
                // Found a sync point, consume it and break
                self.bump().ok();
                break;
            }
            // Skip the current token
            self.bump().ok();
        }
    }

    fn is_sync_point(&self, kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::SEMICOLON |
            SyntaxKind::NEWLINE |
            SyntaxKind::R_BRACE |
            // Statement start tokens
            SyntaxKind::IMPORT_KW |
            SyntaxKind::PUBLIC_KW |
            SyntaxKind::FUNCTION_KW |
            SyntaxKind::IF_KW |
            SyntaxKind::WHILE_KW |
            SyntaxKind::RETURN_KW
        )
    }

    fn synchronize(&mut self) {
        // Skip until we find a synchronization point
        self.skip_until_sync_point();

        // Skip any additional newlines/semicolons to get to the next statement
        while let Some(kind) = self.peek_kind() {
            if matches!(kind, SyntaxKind::SEMICOLON | SyntaxKind::NEWLINE) {
                self.bump().ok();
            } else {
                break;
            }
        }
    }
}
