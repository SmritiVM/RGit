use crate::structures::index::{Index}; 
use crate::components::hash_and_compress;
use std::path::Path;

const INDEX_FILE_PATH: &str = ".wegit/index";

pub fn add(filepath: &str) {
    let data = match std::fs::read(filepath) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filepath, e);
            return; 
        }
    };
    let hash_code = hash_and_compress::calculate_sha1(&data); 

    let mut index = match Index::read_index(INDEX_FILE_PATH) {
        Ok(index) => index, 
        Err(_) => Index::new(),
    };
    index.add_index_object(filepath, &hash_code, "A"); // A to denote that the file has been added but not commited
    if let Err(e) = index.write_index(INDEX_FILE_PATH) {
        eprintln!("Error writing to index file: {}", e);
        return; 
    }
    println!("File '{}' added.", filepath);

    let objects_dir = Path::new(".wegit").join("file_objects");
    hash_and_compress::create_object(&objects_dir, &data, &hash_code);
}