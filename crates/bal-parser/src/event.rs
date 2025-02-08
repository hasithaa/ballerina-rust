//! Parser events for incremental parsing

#[derive(Debug)]
pub enum Event {
    StartNode { kind: crate::SyntaxKind },
    AddToken { kind: crate::SyntaxKind, text: String },
    FinishNode,
} 