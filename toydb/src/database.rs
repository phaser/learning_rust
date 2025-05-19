use std::collections::HashMap;
use crate::persistence::storage::Storage;

pub struct Database {
    hash_map: HashMap<String, String>,
    storage: Storage
}

impl Database {
    pub fn new(path: String) -> Self {
        Self {
            hash_map: HashMap::new(),
            storage: Storage::new(path),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.storage.append(&key, &value);
        self.hash_map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.hash_map.get(&key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use crate::database::Database;

    #[test]
    fn test_put() {
        let dir = TempDir::new("test").unwrap();
        let file_path = dir.path().join("test.db");
        let path = file_path.to_str().unwrap().to_string();
        let mut db = Database::new(path);
        db.put("key".to_string(), "value".to_string());
        db.put("key2".to_string(), "value2".to_string());
        db.put("key".to_string(), "value3".to_string());

        // test that insert called twice with the same key overwrites the value
        assert_eq!(db.get("key".to_string()).unwrap(), "value3".to_string());
    }
}