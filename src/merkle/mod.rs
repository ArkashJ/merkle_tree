use std::collections::HashMap;

use merkle::{MerkleTree};
pub mod merkle;
use serde_json::{json, Value};

#[tokio::test]
async fn make_merkle_tree() {
    let record1_json =  json!({
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

    let record2_json = json!({
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

    let mut merkle_tree = MerkleTree::new();
    let mut records: Vec<HashMap<String, Value>> = Vec::new();
    let record1: HashMap<String, Value>  = serde_json::from_value(record1_json.clone()).unwrap();
    records.push(record1);

    assert_eq!(merkle_tree.get_size(), 0);
    assert_eq!(merkle_tree.add(records.clone()), true);
    assert_eq!(merkle_tree.get_size(), 1);


    let record2: HashMap<String, Value> = serde_json::from_value(record2_json).unwrap();
    records.push(record2);
    assert_eq!(merkle_tree.add(records.clone()), true);

    let repeat_record1: HashMap<String, Value>  = serde_json::from_value(record1_json).unwrap();
    records.push(repeat_record1);
    assert_eq!(merkle_tree.add(records.clone()), false);
    assert_eq!(merkle_tree.get_size(), 2);

    let root = merkle_tree.get_root();
    assert_eq!(root.is_empty(), false);
}