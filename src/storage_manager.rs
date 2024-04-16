use std::collections::HashMap;
use std::hash::{Hasher, DefaultHasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    name: String,
    size: usize,
    file_type: String,
    hash: u64,
}

impl File {
    pub fn new(name: String, size: usize, file_type: String, hash: u64) -> Self {
        Self { name, size, file_type, hash }
    }

    pub fn calculate_hash(contents: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(contents);
        hasher.finish()
    }

    // Public getter methods
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn file_type(&self) -> &str {
        &self.file_type
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    #[allow(dead_code)]
    id: u32,
    files: Vec<File>,
}

impl Node {
    pub fn new(id: u32) -> Self {
        Self { id, files: Vec::new() }
    }

    // Public method to safely access files
    pub fn get_files(&self) -> &Vec<File> {
        &self.files
    }
}

pub struct StorageManager {
    nodes: HashMap<u32, Node>,  // Maps node ID to Node
    file_locations: HashMap<String, Vec<u32>>,  // Maps file name to list of node IDs where it's stored
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            file_locations: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, name: String, contents: &[u8], file_type: String, replicas: usize) {
        let size = contents.len();
        let hash = File::calculate_hash(contents);
        let file = File::new(name.clone(), size, file_type, hash);

        for node_id in 1..=replicas as u32 {
            self.nodes.entry(node_id).or_insert(Node::new(node_id)).files.push(file.clone());
            self.file_locations.entry(name.clone()).or_default().push(node_id);
        }
    }

    pub fn check_redundancy(&self, name: &str) -> Option<Vec<u32>> {
        self.file_locations.get(name).cloned()
    }

    // Public method to access node data securely
    pub fn get_node(&self, node_id: u32) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    pub fn delete_file(&mut self, name: &str) -> bool {
        if let Some(node_ids) = self.file_locations.remove(name) {
            for node_id in node_ids {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.files.retain(|file| file.name != name);
                }
            }
            true
        } else {
            false
        }
    }
}