pub struct ToyDbStorage {
    pub path: String,
}

impl ToyDbStorage {
    pub fn new(path: String) -> Self {
        ToyDbStorage { path }
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use crate::persistence::toydb_storage::ToyDbStorage;

    #[test]
    fn test_create_new_toydb_storage() {
        let dir = TempDir::new("test").unwrap();
        let file_path = dir.path().join("test.db");
        let path = file_path.to_str().unwrap().to_string();
        let toydb_storage = ToyDbStorage::new(path);
        assert_eq!(toydb_storage.path, file_path.to_str().unwrap().to_string());
    }
}