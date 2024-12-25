// use std::path::Path;
// use std::fs::{File, OpenOptions};
// use std::io::{self, Write};
use crate::structures::index::{Index, IndexObject}; // Assuming index.rs is in the structures module
use sha1::{Sha1, Digest};

// The path to the index file
const INDEX_FILE_PATH: &str = ".wegit/index";

pub fn add(filepath: &str) {
    // Step 1: Try to read the existing index from the file
    let mut index = match Index::read_index(INDEX_FILE_PATH) {
        Ok(index) => index, // If the index file exists, read it
        Err(_) => Index::new(), // If the index file doesn't exist, create a new index
    };

    // Step 2: Try to calculate the SHA1 hash of the file
    let hash_code = match calculate_sha1(filepath) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Error calculating SHA1 for '{}': {}", filepath, e);
            return; // If SHA1 calculation fails, we exit early
        }
    };

    // Step 3: Add the new file (with hash) to the index
    index.add_index_object(filepath, &hash_code);

    // Step 4: Try to write the updated index back to the file
    if let Err(e) = index.write_index(INDEX_FILE_PATH) {
        eprintln!("Error writing to index file: {}", e);
        return; // Exit if writing to the index file fails
    }

    // Step 5: Print a success message
    println!("File '{}' added to index.", filepath);
}

fn calculate_sha1(file_path: &str) -> std::io::Result<String> {
    let mut hasher = Sha1::new();
    let data = std::fs::read(file_path)?;

    hasher.update(data);
    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}