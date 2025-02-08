//! Grammar rules for Ballerina parser

use super::Parser;
use bal_syntax::{BallerinaLanguage, SyntaxKind};
use rowan::Language;

impl Parser {
    pub(crate) fn parse_function(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::FUNCTION_DEF));
        // Parse function declaration and body
        self.builder.finish_node();
    }

    pub(crate) fn parse_parameter_list(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::PARAM_LIST));
        // Parse parameters
        self.builder.finish_node();
    }
} 