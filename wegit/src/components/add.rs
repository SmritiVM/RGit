use crate::structures::index::{Index}; 
use crate::components::hash_and_compress;
use crate::structures::paths;
use std::path::Path;

pub fn add(filepath: &str) {
    let data = match std::fs::read(filepath) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Oops! File doesn't exist in this directory");
            return; 
        }
    };
    let hash_code = hash_and_compress::calculate_sha1(&data); 

    let mut index = match Index::read_index(paths::INDEX) {
        Ok(index) => index, 
        Err(_) => Index::new(),
    };
    index.add_index_object(filepath, &hash_code);
    if let Err(e) = index.write_index(paths::INDEX) {
        eprintln!("Fatal, not a git repo!");
        return; 
    }
    println!("File '{}' added.", filepath);
    hash_and_compress::create_object(Path::new(paths::FILE_OBJECTS), &data, &hash_code);
}