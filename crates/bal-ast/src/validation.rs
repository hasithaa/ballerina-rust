//! AST validation rules and error reporting

pub struct ValidationError {
    pub message: String,
    pub span: Option<rowan::TextRange>,
}

pub fn validate_ast(ast: &crate::SourceFile) -> Vec<ValidationError> {
    Vec::new() // TODO: Implement validation rules
} 