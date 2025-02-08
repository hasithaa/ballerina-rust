//! Ballerina Parser Library
//! Handles incremental parsing and syntax tree construction

use bal_syntax::SyntaxKind;
use rowan::{GreenNode, GreenNodeBuilder};
use bal_syntax::error::{ParserError, Span};

pub type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser {
    builder: GreenNodeBuilder<'static>,
    tokens: Vec<(SyntaxKind, String, Span)>,
    cursor: usize,
    file: Option<String>,
}

impl Parser {
    pub fn new(file: Option<String>, tokens: Vec<(SyntaxKind, String, Span)>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            file,
        }
    }

    pub fn parse(mut self) -> Result<GreenNode> {
        self.parse_module_part()?;
        Ok(self.builder.finish())
    }
}

pub mod event;
pub mod grammar;
pub mod sink;

#[cfg(test)]
mod parser_test; 