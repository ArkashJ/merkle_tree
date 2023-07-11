mod merkle;
use merkle_tree::merkle::merkle::MerkleTree;
use serde_json::{json, Value, Map};
use std::collections::{HashMap};

fn main(){
    let mut merkle_tree = MerkleTree::new();
    let mut records: Vec<HashMap<String, Value>> = Vec::new();

    let record1 = json!({
        "_id": "q384hgjkasdfnlv",
        "fullName": "darth vader",
        "yearsOld": 150,
        "emailAddress": "bob@aol.com",
        "address": {
            "streetAddress": "15 park drive",
            "town": "boston",
            "state": "massachussets",
            "zip": 12345,
        },
        "phoneNumber": 1234567892,
        "isAdmin": true,
    });

    let record2 = json!({
        "_id": "q384hgjkasdfnlv2",
        "fullName": "darth maul",
        "yearsOld": 120,
        "emailAddress": "maul@aol.com",
        "address": {
            "streetAddress": "15 park drive",
            "town": "boston",
            "state": "massachussets",
            "zip": 12345,
        },
        "phoneNumber": 1234567892,
        "isAdmin": true,
    });
    let record1: HashMap<String, Value>  = serde_json::from_value(record1).unwrap();
    let record2: HashMap<String, Value> = serde_json::from_value(record2).unwrap();

    records.push(record1);
    records.push(record2); 

    merkle_tree.add(records);
    
    merkle_tree.print_tree();
}