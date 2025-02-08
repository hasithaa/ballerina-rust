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
        self.parse_source_file();
        self.builder.finish()
    }

    fn parse_source_file(&mut self) {
        self.builder.start_node(BallerinaLanguage::kind_to_raw(SyntaxKind::SOURCE_FILE));
        // Parse functions and other top-level declarations
        self.builder.finish_node();
    }
}

pub mod event;
pub mod grammar;
pub mod sink; 