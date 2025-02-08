use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(rename = "package")]
    pub info: PackageInfo,
    #[serde(rename = "build-options", default)]
    pub build_options: BuildOptions,
}

#[derive(Debug, Deserialize)]
pub struct PackageInfo {
    pub org: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub distribution: String,
}

#[derive(Debug, Deserialize, Default)]
#[allow(non_snake_case)] // Ballerina.toml uses snake_case for field names
pub struct BuildOptions {
    #[serde(default)]
    pub observabilityIncluded: bool,
}

#[derive(Debug)]
pub struct Project {
    pub root_dir: PathBuf,
    pub package: Package,
    pub source_files: Vec<PathBuf>,
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("Invalid project: {0}")]
    Invalid(String),
}

impl Project {
    pub fn load(path: &Path) -> Result<Self, ProjectError> {
        // Determine the root directory based on whether the path is a file or directory
        let root_dir = if path.is_file() {
            path.parent()
                .ok_or_else(|| ProjectError::Invalid("Invalid project path".to_string()))?
                .to_path_buf()
        } else {
            path.to_path_buf()
        };

        // Read and parse Ballerina.toml
        let toml_path = find_ballerina_toml(&root_dir)
            .ok_or_else(|| ProjectError::Invalid("No Ballerina.toml found".to_string()))?;
        
        let toml_content = fs::read_to_string(&toml_path)?;
        let package: Package = toml::from_str(&toml_content)?;

        // Find all .bal files
        let source_files = find_bal_files(&root_dir);
        if source_files.is_empty() {
            return Err(ProjectError::Invalid("No .bal files found in project".to_string()));
        }

        Ok(Project {
            root_dir,
            package,
            source_files,
        })
    }

    pub fn is_valid_source_file(&self, path: &Path) -> bool {
        path.extension().and_then(|ext| ext.to_str()) == Some("bal") 
            && path.parent() == Some(&self.root_dir)
    }
}

pub fn find_ballerina_toml(dir: &Path) -> Option<PathBuf> {
    let toml_path = dir.join("Ballerina.toml");
    if toml_path.exists() {
        Some(toml_path)
    } else {
        let lowercase_toml = dir.join("ballerina.toml");
        if lowercase_toml.exists() {
            Some(lowercase_toml)
        } else {
            None
        }
    }
}

fn find_bal_files(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("bal"))
        .collect()
} 