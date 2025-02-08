//! Ballerina AST Library
//! Handles AST processing and semantic analysis

use bal_syntax::SyntaxKind;
use rowan::SyntaxNode;

#[derive(Debug)]
pub struct SourceFile {
    syntax: SyntaxNode<bal_syntax::BallerinaLanguage>,
}

impl SourceFile {
    pub fn new(syntax: SyntaxNode<bal_syntax::BallerinaLanguage>) -> Self {
        Self { syntax }
    }

    pub fn functions(&self) -> impl Iterator<Item = Function> {
        self.syntax
            .children()
            .filter_map(|node| Function::cast(node))
    }
}

#[derive(Debug)]
pub struct Function {
    syntax: SyntaxNode<bal_syntax::BallerinaLanguage>,
}

impl Function {
    pub fn cast(node: SyntaxNode<bal_syntax::BallerinaLanguage>) -> Option<Self> {
        if node.kind() == SyntaxKind::FUNCTION_DEF {
            Some(Self { syntax: node })
        } else {
            None
        }
    }
}

pub mod semantic;
pub mod types;
pub mod validation; 