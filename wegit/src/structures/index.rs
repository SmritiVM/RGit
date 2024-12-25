use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::collections::HashMap;
// use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct IndexObject {
    pub filepath: String,
    pub hash_code: String, 
}

impl IndexObject {
    pub fn new(filepath: &str, hash_code: &str) -> Self {
        IndexObject {
            filepath: filepath.to_string(),
            hash_code: hash_code.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Index {
    pub objects: HashMap<String, IndexObject>, 
}

impl Index {
    pub fn new() -> Self {
        Index {
            objects: HashMap::new(),
        }
    }

    pub fn add_index_object(&mut self, filepath: &str, hash_code: &str) {
        let index_object = IndexObject::new(filepath, hash_code);
        self.objects.insert(filepath.to_string(), index_object);
    }

    pub fn read_index(file_path: &str) -> std::io::Result<Index> {
        let mut index = Index::new();
        let mut file = File::open(file_path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let filepath = parts[0].to_string();
                let hash_code = parts[1].to_string();
                index.add_index_object(&filepath, &hash_code);
            }
        }

        Ok(index)
    }

    pub fn write_index(&self, file_path: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?;

        for index_object in self.objects.values() {
            writeln!(file, "{} {}", index_object.filepath, index_object.hash_code)?;
        }

        Ok(())
    }

}


