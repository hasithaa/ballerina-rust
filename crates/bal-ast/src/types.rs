//! Type system for Ballerina AST

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    NIL,
    // Add more types as needed
} 