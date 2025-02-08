//! Ballerina Parser Library
//! Handles incremental parsing and syntax tree construction

use bal_syntax::{BallerinaLanguage, SyntaxKind};
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub struct Parser {
    builder: GreenNodeBuilder<'static>,
    tokens: Vec<(SyntaxKind, String)>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(SyntaxKind, String)>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
        }
    }

    pub fn parse(mut self) -> GreenNode {
        self.parse_module_part();
        self.builder.finish()
    }

    // fn parse_source_file(&mut self) { ... }
}

pub mod event;
pub mod grammar;
pub mod sink;

#[cfg(test)]
mod parser_test; 