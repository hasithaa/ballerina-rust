use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use bal_syntax::project::Project;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Map from module ID to its dependencies
    pub dependencies: HashMap<ModuleId, HashSet<ModuleId>>,
    /// Map from file path to last modified time
    timestamps: HashMap<PathBuf, SystemTime>,
    /// Map from module ID to file path
    pub module_files: HashMap<ModuleId, PathBuf>,
    /// Map from file path to cached parse results and source hash
    #[serde(skip)]  // Don't serialize parse results
    parse_cache: HashMap<PathBuf, (rowan::GreenNode, u64)>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModuleId {
    org: String,
    name: String,
    version: String,
    module: String,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            timestamps: HashMap::new(),
            module_files: HashMap::new(),
            parse_cache: HashMap::new(),
        }
    }

    /// Load cached dependency graph if it exists and is up to date
    pub fn load_cached(project: &Project) -> Option<Self> {
        let cache_path = project.root_dir.join("target").join("deps.cache");
        
        // Try to read cached data
        if let Ok(cached_data) = std::fs::read(&cache_path) {
            if let Ok(graph) = bincode::deserialize::<DependencyGraph>(&cached_data) {
                // Verify all timestamps are still valid
                if graph.is_up_to_date() {
                    return Some(graph);
                }
            }
        }
        None
    }

    /// Save dependency graph to cache
    pub fn save_cache(&self, project: &Project) -> std::io::Result<()> {
        let cache_dir = project.root_dir.join("target");
        std::fs::create_dir_all(&cache_dir)?;
        
        let cache_path = cache_dir.join("deps.cache");
        let data = bincode::serialize(self).unwrap();
        std::fs::write(cache_path, data)
    }

    /// Check if cached data is still valid
    fn is_up_to_date(&self) -> bool {
        for (path, cached_time) in &self.timestamps {
            match path.metadata().and_then(|m| m.modified()) {
                Ok(current_time) => {
                    if &current_time != cached_time {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }
        true
    }

    /// Add a module and its dependencies to the graph
    pub fn add_module(&mut self, module: ModuleId, dependencies: HashSet<ModuleId>, file_path: PathBuf) {
        self.dependencies.insert(module.clone(), dependencies);
        self.module_files.insert(module, file_path.clone());
        
        // Update timestamp
        if let Ok(metadata) = std::fs::metadata(&file_path) {
            if let Ok(modified) = metadata.modified() {
                self.timestamps.insert(file_path, modified);
            }
        }
    }

    /// Get sorted list of modules in build order (topological sort)
    pub fn build_order(&self) -> Vec<ModuleId> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp = HashSet::new();

        for module in self.dependencies.keys() {
            if !visited.contains(module) {
                self.visit_module(module, &mut visited, &mut temp, &mut result);
            }
        }

        result
    }

    fn visit_module(
        &self,
        module: &ModuleId,
        visited: &mut HashSet<ModuleId>,
        temp: &mut HashSet<ModuleId>,
        result: &mut Vec<ModuleId>
    ) {
        if temp.contains(module) {
            panic!("Circular dependency detected");
        }
        if visited.contains(module) {
            return;
        }

        temp.insert(module.clone());

        if let Some(deps) = self.dependencies.get(module) {
            for dep in deps {
                self.visit_module(dep, visited, temp, result);
            }
        }

        temp.remove(module);
        visited.insert(module.clone());
        result.push(module.clone());
    }

    /// Get cached parse result if source hasn't changed
    pub fn get_cached_parse(&self, path: &Path, source: &str) -> Option<&rowan::GreenNode> {
        let current_hash = calculate_hash(source);
        self.parse_cache.get(path)
            .filter(|(_, hash)| *hash == current_hash)
            .map(|(tree, _)| tree)
    }

    /// Cache parse result for a file
    pub fn cache_parse(&mut self, path: PathBuf, source: &str, tree: rowan::GreenNode) {
        let hash = calculate_hash(source);
        self.parse_cache.insert(path, (tree, hash));
    }
}

impl std::fmt::Display for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.org, self.name, self.module)
    }
}

/// Build dependency graph for a project
pub fn build_project_dependencies(
    project: &Project,
    config: &Config,
) -> std::io::Result<(DependencyGraph, bool)> {
    // Try loading cached graph first
    if let Some(graph) = DependencyGraph::load_cached(project) {
        config.debug("Using cached dependency graph");
        return Ok((graph, true));
    }

    config.debug("Building new dependency graph");
    let mut graph = DependencyGraph::new();
    
    // Parse all files to build dependency graph
    for file_path in &project.source_files {
        let source = std::fs::read_to_string(file_path)?;
        let imports = parse_imports(&source)?;
        
        let module_id = module_id_from_path(project, file_path);
        let dependencies = imports.into_iter()
            .map(|imp| ModuleId::from_import(&imp))
            .collect();
            
        graph.add_module(module_id, dependencies, file_path.to_path_buf());
    }

    // Cache the graph
    graph.save_cache(project)?;
    Ok((graph, false))
}

/// Parse import statements from source code
fn parse_imports(source: &str) -> std::io::Result<Vec<String>> {
    // TODO: Use actual parser to extract imports
    // For now, just a simple example implementation
    let mut imports = Vec::new();
    for line in source.lines() {
        if line.trim().starts_with("import ") {
            imports.push(line.trim()[7..].trim_end_matches(';').to_string());
        }
    }
    Ok(imports)
}

fn module_id_from_path(project: &Project, path: &Path) -> ModuleId {
    // TODO: Extract module info from path relative to project
    ModuleId {
        org: project.package.info.org.clone(),
        name: project.package.info.name.clone(),
        version: project.package.info.version.clone(),
        module: path.file_stem().unwrap().to_string_lossy().to_string(),
    }
}

impl ModuleId {
    fn from_import(import_path: &str) -> Self {
        // Simple implementation - split on '/'
        let parts: Vec<&str> = import_path.split('/').collect();
        Self {
            org: parts.get(0).unwrap_or(&"").to_string(),
            name: parts.get(1).unwrap_or(&"").to_string(),
            version: "0.1.0".to_string(), // Default version
            module: parts.get(2).unwrap_or(&"").to_string(),
        }
    }
}

/// Calculate a hash of the source code for change detection
fn calculate_hash(source: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
} 