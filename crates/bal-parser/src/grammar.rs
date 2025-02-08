//! Grammar rules for Ballerina parser

use super::Parser;
use bal_syntax::{BallerinaLanguage, SyntaxKind};
use rowan::Language;

impl Parser {
    pub(crate) fn parse_module_part(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::MODULE_PART));
        
        // Optional import declaration
        if self.at(SyntaxKind::IMPORT_KW) {
            self.parse_import_decl();
        }
        
        // Zero or more module declarations
        while !self.at_end() {
            self.parse_module_decl();
        }
        
        self.builder.finish_node();
    }

    fn parse_import_decl(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::IMPORT_DECL));
        
        self.expect(SyntaxKind::IMPORT_KW);
        self.expect(SyntaxKind::IDENTIFIER);
        self.expect(SyntaxKind::SLASH);
        self.expect(SyntaxKind::IDENTIFIER);
        self.expect(SyntaxKind::SEMICOLON);
        
        self.builder.finish_node();
    }

    fn parse_module_decl(&mut self) {
        // Currently only function definitions are supported
        self.parse_function_def();
    }

    fn parse_function_def(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::FUNCTION_DEF));
        
        // Optional public modifier
        if self.at(SyntaxKind::PUBLIC_KW) {
            self.bump();
        }
        
        self.expect(SyntaxKind::FUNCTION_KW);
        self.expect(SyntaxKind::IDENTIFIER);
        self.parse_signature();
        self.parse_stmt_block();
        
        self.builder.finish_node();
    }

    fn parse_signature(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::SIGNATURE));
        
        self.expect(SyntaxKind::L_PAREN);
        if !self.at(SyntaxKind::R_PAREN) {
            self.parse_param_list();
        }
        self.expect(SyntaxKind::R_PAREN);
        
        if self.at(SyntaxKind::RETURNS_KW) {
            self.bump();
            self.parse_type_desc();
        }
        
        self.builder.finish_node();
    }

    fn parse_param_list(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::PARAM_LIST));
        
        loop {
            self.parse_param();
            if !self.at(SyntaxKind::COMMA) {
                break;
            }
            self.bump(); // Consume comma
        }
        
        self.builder.finish_node();
    }

    fn parse_param(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::PARAM));
        self.parse_type_desc();
        self.expect(SyntaxKind::IDENTIFIER);
        self.builder.finish_node();
    }

    fn parse_type_desc(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::TYPE_DESC));
        self.expect_one_of(&[SyntaxKind::INT_KW, SyntaxKind::BOOLEAN_KW]);
        self.builder.finish_node();
    }

    fn parse_stmt_block(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::STMT_BLOCK));
        self.expect(SyntaxKind::L_BRACE);
        // TODO: Parse statements here
        self.expect(SyntaxKind::R_BRACE);
        self.builder.finish_node();
    }

    // Helper methods
    fn at(&self, kind: SyntaxKind) -> bool {
        self.peek_kind() == Some(kind)
    }

    fn at_end(&self) -> bool {
        self.peek_kind().is_none()
    }

    fn peek_kind(&self) -> Option<SyntaxKind> {
        if self.cursor < self.tokens.len() {
            Some(self.tokens[self.cursor].0)
        } else {
            None
        }
    }

    fn bump(&mut self) {
        if self.cursor < self.tokens.len() {
            let (kind, text) = self.tokens[self.cursor].clone();
            self.builder.token(BallerinaLanguage::kind_to_raw(kind), &text);
            self.cursor += 1;
        }
    }

    fn expect(&mut self, kind: SyntaxKind) {
        if self.at(kind) {
            self.bump();
        } else {
            // TODO: Handle error recovery
            self.builder.token(BallerinaLanguage::kind_to_raw(SyntaxKind::ERROR), "");
        }
    }

    fn expect_one_of(&mut self, kinds: &[SyntaxKind]) {
        if let Some(current) = self.peek_kind() {
            if kinds.contains(&current) {
                self.bump();
                return;
            }
        }
        // TODO: Handle error recovery
        self.builder.token(BallerinaLanguage::kind_to_raw(SyntaxKind::ERROR), "");
    }
} 