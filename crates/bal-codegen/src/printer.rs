//! Source code pretty printing

pub struct Printer {
    indent: usize,
}

impl Printer {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    pub fn print(&self, source: &str) -> String {
        // TODO: Implement pretty printing
        source.to_string()
    }
}
