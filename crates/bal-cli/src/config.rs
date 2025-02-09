//! CLI configuration settings

#[derive(Debug, Default)]
pub struct Config {
    pub debug: bool,
}

impl Config {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    pub fn debug(&self, msg: &str) {
        if self.debug {
            eprintln!("DEBUG: {}", msg);
        }
    }
}
