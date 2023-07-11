use sha2::{Digest, Sha256};
use std::collections::{HashSet, HashMap};
use serde_json::Value;

// #[derive(Debug, Clone)]
// pub struct Record {
//     pub name: String,
//     pub id: String,
// }

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Option<String>,
    pub nodes: HashSet<String>,
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree {
            root: None,
            nodes: HashSet::new(),
        }
    }

    pub fn make_hash(records: &HashMap<String, Value>) -> String {
        let mut fields: Vec<String> = records
            .iter()
            .map(|(key, value)| {format!("{}{}", key, Self::value_to_str(value))})
            .collect();
        fields.sort();

        let record_str = fields.iter().fold(String::new(), |mut acc, field| {
            acc.push_str(field);
            acc
        });

        let mut hasher: Sha256 = Sha256::new();
        hasher.update(record_str);
        let result = hasher.finalize();
        let hash_string = format!("{:x}", result);
        hash_string
    }

    fn value_to_str(value: &Value) -> String{
        match value {
            Value::String(s) => s.to_string(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(a) => a.iter().fold(String::new(), |mut acc, v| {
                acc.push_str(&Self::value_to_str(v));
                acc
            }),
            Value::Object(o) => o.iter().fold(String::new(), |mut acc, (k, v)| {
                acc.push_str(k);
                acc.push_str(&Self::value_to_str(v));
                acc
            }),
        }
    }
    fn make_hash_str(data: &str) -> String {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let hash_string = format!("{:x}", result);
        hash_string
    }

    pub fn get_size(&self) -> usize {
        self.nodes.len()
    }

    pub fn add(&mut self, records: Vec<HashMap<String, Value>>) -> bool {
        let mut hashes: Vec<String> = records
            .iter()
            .map(|record| {
                let hash = Self::make_hash(record);
                if self.nodes.contains(&hash) {
                    return None;
                }
                self.nodes.insert(hash.clone());
                Some(hash)
            })
            .filter(|hash| hash.is_some())
            .map(|hash| hash.unwrap())
            .collect();
        if hashes.is_empty() {
            println!("No new records to add");
            return false;
        }
        let mut hashes_len = hashes.len();
        while hashes_len > 1 {
            let mut layer = vec![];
            for pair in hashes.chunks(2) {
                let mut pair = pair.to_vec();
                pair.sort();
                let node = pair.iter().fold(String::new(), |mut acc, hash| {
                    acc.push_str(hash);
                    acc
                });
                layer.push(Self::make_hash_str(&node));
            }
            hashes = layer;
            hashes_len = hashes.len();
        }
        self.root = Some(hashes[0].clone());
        true
    }

    pub fn verify(&self, record: &HashMap<String, Value>) -> bool {
        let hash_string = Self::make_hash(record);
        self.nodes.contains(&hash_string)
    }

    pub fn verify_root(&self, root: String) -> bool {
        self.root == Some(root)
    }

    pub fn get_root(&self) -> String {
        self.root.clone().unwrap_or_default()
    }

    pub fn print_tree(&self) {
        println!("Merkle Tree:");
        println!("Root: {:?}", self.root);
        println!("Nodes: {:?}", self.nodes);
    }
}
