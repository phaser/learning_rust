use std::fs::File;
use std::io::{Read, Write};
use std::fs::OpenOptions;

pub struct Storage {
    pub path: String,
}

impl Storage {
    pub fn new(path: String) -> Self {
        Storage { path }
    }

    pub fn append(&self, key: &String, value: &String) {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect("Unable to open database file.");
        f.write_all(&key.len().to_be_bytes()).unwrap();
        f.write_all(key.as_bytes()).unwrap();
        f.write_all(&value.len().to_be_bytes()).unwrap();
        f.write_all(value.as_bytes()).unwrap()
    }

    pub fn get_mut_file(&self) -> File {
        File::open(&self.path).unwrap()
    }

    pub fn get_key_value(&self, file: &mut File) -> (String, String) {
        (Self::read_key(file), Self::read_key(file))
    }

    fn read_key(file: &mut File) -> String {
        let mut fsize: [u8; 8] = [0; 8];
        file.read(&mut fsize).unwrap();
        let n = usize::from_be_bytes(*&fsize);
        let mut key = vec![0u8; n];
        file.read_exact(&mut key).unwrap();
        String::from_utf8(key).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use crate::persistence::storage::Storage;

    #[test]
    fn test_create_new_toydb_storage() {
        let dir = TempDir::new("test").unwrap();
        let file_path = dir.path().join("test.db");
        let path = file_path.to_str().unwrap().to_string();
        let toydb_storage = Storage::new(path);
        assert_eq!(toydb_storage.path, file_path.to_str().unwrap().to_string());
    }

    #[test]
    fn test_simple_append() {
        let dir = TempDir::new("test").unwrap();
        let file_path = dir.path().join("test.db");
        let path = file_path.to_str().unwrap().to_string();
        let db = Storage::new(path);
        db.append(&"test".to_string(), &"value".to_string());
        db.append(&"test1".to_string(), &"value1".to_string());

        let mut f = db.get_mut_file();
        let (key, value) = db.get_key_value(&mut f);
        assert_eq!(key, "test".to_string());
        assert_eq!(value, "value".to_string());
        let (key, value) = db.get_key_value(&mut f);
        assert_eq!(key, "test1".to_string());
        assert_eq!(value, "value1".to_string());
    }
}