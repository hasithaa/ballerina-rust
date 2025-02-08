use super::*;
use std::path::PathBuf;

fn test_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ballerina-src")
        .join("m1")
}

#[test]
fn test_build_single_file() {
    let path = test_dir().join("exp1.bal");
    let result = build(Some(path));
    assert!(result.is_ok(), "Failed to build single file: {:?}", result);
}

#[test]
fn test_build_file_in_project() {
    let path = test_dir()
        .join("projects")
        .join("proj1")
        .join("modules")
        .join("abc")
        .join("abc.bal");
    
    // Should fail since file is part of a project
    let result = build(Some(path));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("is in a Ballerina project directory"));
}

#[test]
fn test_build_project() {
    let path = test_dir().join("projects").join("proj1");
    let result = build(Some(path));
    assert!(result.is_ok(), "Failed to build project: {:?}", result);
}

#[test]
fn test_build_nonexistent_file() {
    let path = test_dir().join("nonexistent.bal");
    let result = build(Some(path));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));
}

#[test]
fn test_build_invalid_extension() {
    let path = test_dir().join("invalid.txt");
    let result = build(Some(path));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Not a Ballerina file"));
}

#[test]
fn test_build_project_submodule() {
    let path = test_dir()
        .join("projects")
        .join("proj1")
        .join("modules")
        .join("abc");
    
    let result = build(Some(path));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid project"));
}

#[test]
fn test_build_current_directory() {
    // First change to the test project directory
    let project_dir = test_dir().join("projects").join("proj1");
    std::env::set_current_dir(&project_dir).unwrap();
    
    let result = build(None);
    assert!(result.is_ok(), "Failed to build current directory: {:?}", result);
}

// Helper function to create test files for setup/teardown if needed
#[allow(dead_code)]
fn setup_test_files() -> std::io::Result<()> {
    let test_base = test_dir();
    
    // Create single file
    std::fs::write(
        test_base.join("exp1.bal"),
        "public function main() { }\n"
    )?;
    
    // Create project structure
    let proj_dir = test_base.join("projects").join("proj1");
    std::fs::create_dir_all(&proj_dir)?;
    
    std::fs::write(
        proj_dir.join("Ballerina.toml"),
        r#"[package]
org = "test"
name = "proj1"
version = "0.1.0"
"#
    )?;
    
    // Create module
    let module_dir = proj_dir.join("modules").join("abc");
    std::fs::create_dir_all(&module_dir)?;
    
    std::fs::write(
        module_dir.join("abc.bal"),
        "public function hello() { }\n"
    )?;
    
    Ok(())
}

// Helper function for cleanup if needed
#[allow(dead_code)]
fn cleanup_test_files() -> std::io::Result<()> {
    let test_base = test_dir();
    std::fs::remove_file(test_base.join("exp1.bal"))?;
    std::fs::remove_dir_all(test_base.join("projects"))?;
    Ok(())
} 