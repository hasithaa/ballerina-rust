//! CLI command implementations

use std::fs;
use std::path::{Path, PathBuf};
use bal_parser::Parser;
use bal_syntax::lexer::Lexer;
use bal_syntax::project::Project;
use crate::dependency::{build_project_dependencies, DependencyGraph, ModuleId};
use crate::config::Config;
use std::collections::{HashMap, HashSet};


fn is_bal_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| ext == "bal")
}

pub fn build(input: Option<PathBuf>, config: &Config) -> Result<(), String> {
    match input {
        Some(path) => {
            if path.is_dir() {
                build_project_from_path(&path, config)
            } else {
                build_single_file(&path, config)
            }
        }
        None => build_project_from_path(
            &std::env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?,
            config,
        )
    }
}

fn build_single_file(path: &Path, config: &Config) -> Result<(), String> {
    // Validate file exists
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    // Validate file extension before checking project
    if !is_bal_file(path) {
        return Err(format!("Not a Ballerina file: {}", path.display()));
    }

    // Check all parent directories for Ballerina.toml
    let mut current_dir = path.parent();
    while let Some(dir) = current_dir {
        if let Some(toml_path) = bal_syntax::project::find_ballerina_toml(dir) {
            return Err(format!(
                "File {} is in a Ballerina project directory (found {} in {}). Use 'bal build' in the project directory.",
                path.display(),
                toml_path.file_name().unwrap_or_default().to_string_lossy(),
                dir.display()
            ));
        }
        current_dir = dir.parent();
    }

    let mut dep_graph = DependencyGraph::new();
    parse_and_build_file(path, config, &mut dep_graph)
}

fn build_project_from_path(project_path: &Path, config: &Config) -> Result<(), String> {
    let project = Project::load(project_path)
        .map_err(|e| format!("Failed to load project: {}", e))?;

    println!("Building Ballerina project: {}/{} v{}", 
        project.package.info.org,
        project.package.info.name, 
        project.package.info.version);

    config.debug(&format!("Project directory: {}", project.root_dir.display()));

    let (mut dep_graph, from_cache) = build_project_dependencies(&project, config)
        .map_err(|e| format!("Failed to build dependency graph: {}", e))?;

    config.debug(&format!("Dependency graph {} from cache", 
        if from_cache { "loaded" } else { "built" }));

    // Get all the data we need before mutable operations
    let build_tasks: Vec<_> = dep_graph.build_order()
        .into_iter()
        .filter_map(|id| {
            dep_graph.module_files.get(&id)
                .map(|path| (id, path.clone()))
        })
        .collect();

    // Now do the mutable operations
    let mut project_had_errors = false;
    for (_module_id, file_path) in build_tasks {
        config.debug(&format!("\nBuilding file: {}", file_path.display()));
        if let Err(e) = parse_and_build_file(&file_path, config, &mut dep_graph) {
            eprintln!("Error in {}: {}", file_path.display(), e);
            project_had_errors = true;
        }
    }

    // Print dependency tree only in debug mode
    config.debug("\nDependency Tree:");
    if config.debug {
        print_dependency_tree(&dep_graph.dependencies);
    }

    if project_had_errors {
        Err("Project build completed with errors".to_string())
    } else {
        println!("Build successful");
        Ok(())
    }
}

/// Print dependency tree with nice formatting
fn print_dependency_tree(deps: &HashMap<ModuleId, HashSet<ModuleId>>) {
    for (module, dependencies) in deps {
        println!("{}:", module);
        for dep in dependencies {
            println!("  └─ {}", dep);
        }
    }
}

fn parse_and_build_file(path: &Path, config: &Config, dep_graph: &mut DependencyGraph) -> Result<(), String> {
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Check if we have a valid cached parse tree
    if let Some(parse_tree) = dep_graph.get_cached_parse(path, &source) {
        config.debug(&format!("Using cached parse tree for {}", path.display()));
        println!("Parse tree:\n{:#?}", parse_tree);
        return Ok(());
    }

    // If no cache hit, parse the file
    config.debug(&format!("Parsing file {}", path.display()));
    
    // Tokenize with error handling
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&source);
    let mut had_errors = false;
    
    while let Some(result) = lexer.next_token() {
        match result {
            Ok(token_info) => {
                if !matches!(token_info.kind, bal_syntax::lexer::Token::Newline) {
                    let kind = bal_syntax::convert_token(token_info.kind);
                    tokens.push((kind, token_info.text, token_info.span));
                }
            }
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                had_errors = true;
            }
        }
    }

    let file_name = path.to_str().map(String::from);
    let parser = Parser::new(file_name, tokens);
    match parser.parse() {
        Ok(parse_tree) => {
            config.debug(&format!("Parse tree:\n{:#?}", parse_tree));
            
            // Cache successful parse
            dep_graph.cache_parse(path.to_path_buf(), &source, parse_tree);
            
            if had_errors {
                Err("Completed with errors".to_string())
            } else {
                config.debug(&format!("Successfully parsed: {}", path.display()));
                Ok(())
            }
        }
        Err(e) => {
            eprintln!("Parser error: {}", e);
            Err("Parser errors encountered".to_string())
        }
    }
}

pub fn clean(path: Option<PathBuf>, config: &Config) -> Result<(), String> {
    // Get project path
    let project_path = match path {
        Some(p) => {
            if !p.is_dir() {
                return Err("Clean command only works on project directories".to_string());
            }
            p
        }
        None => std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?,
    };

    // Verify it's a Ballerina project
    if bal_syntax::project::find_ballerina_toml(&project_path).is_none() {
        return Err(format!(
            "Directory {} is not a Ballerina project (no Ballerina.toml found)",
            project_path.display()
        ));
    }

    // Delete target directory
    let target_dir = project_path.join("target");
    if target_dir.exists() {
        config.debug(&format!("Removing target directory: {}", target_dir.display()));
        std::fs::remove_dir_all(&target_dir)
            .map_err(|e| format!("Failed to remove target directory: {}", e))?;
        println!("Cleaned target directory");
    } else {
        println!("Target directory does not exist, nothing to clean");
    }

    Ok(())
}

#[cfg(test)]
mod tests; 