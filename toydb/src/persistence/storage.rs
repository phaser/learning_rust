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
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect("Unable to open database file.");
        File::open(&self.path).unwrap()
    }

    pub fn get_key_value(&self, file: &mut File) -> Option<(String, String)> {
        let key = Self::read_key(file)?;
        let value = Self::read_key(file)?;
        Some((key, value))
    }

    fn read_key(file: &mut File) -> Option<String> {
        let mut fsize: [u8; 8] = [0; 8];
        let no_bytes_read = file.read(&mut fsize).ok()?;
        if no_bytes_read != 8 {
            return None;
        }
        let n = usize::from_be_bytes(*&fsize);
        let mut key = vec![0u8; n];
        file.read_exact(&mut key).ok()?;
        String::from_utf8(key).ok()
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
        let mut f = db.get_mut_file();
        let res = db.get_key_value(&mut f);
        assert_eq!(res, None);

        db.append(&"test".to_string(), &"value".to_string());
        db.append(&"test1".to_string(), &"value1".to_string());

        let (key, value) = db.get_key_value(&mut f).unwrap();
        assert_eq!(key, "test".to_string());
        assert_eq!(value, "value".to_string());
        let (key, value) = db.get_key_value(&mut f).unwrap();
        assert_eq!(key, "test1".to_string());
        assert_eq!(value, "value1".to_string());
        
        let res = db.get_key_value(&mut f);
        assert_eq!(res, None);
    }


}