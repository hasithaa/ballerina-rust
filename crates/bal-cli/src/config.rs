//! CLI configuration settings

pub struct Config {
    pub debug: bool,
    pub optimize: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            optimize: true,
        }
    }
} 