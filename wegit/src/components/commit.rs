use crate::components::hash_and_compress;
use crate::structures::commit;
use crate::structures::paths;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn commit_changes(commit_message: &str) {
    let commit_id = match get_current_commit_id() {
        Some(id) => id + 1,
        None => {
            eprintln!("Failed to get commit ID.");
            return
        }
    };

    update_head(commit_id);

    let index_data = match std::fs::read(paths::INDEX){
        Ok(index_data) => index_data,
        Err(e) => {
            eprintln!("Error: {}", e);
            return; 
        }
    };

    let index_hash = hash_and_compress::calculate_sha1(&index_data); 
    commit::create_commit(commit_id, &index_hash, commit_message);
    hash_and_compress::create_object(Path::new(paths::INDEX_OBJECTS), &index_data, &index_hash)
}

fn get_current_commit_id() -> Option<u64> {
    let mut index_content = String::new();  
    let commit_id: u64 = match File::open(paths::HEAD)
        .and_then(|mut file| file.read_to_string(&mut index_content))  
    {
        Ok(_) => match index_content.trim().parse::<u64>() {
            Ok(id) => id,  
            Err(e) => {
                eprintln!("Error: {}", e);
                return None; 
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);  
            return None;
        }
    };
    Some(commit_id)
}

fn update_head(commit_id: u64) {
    let mut head_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(paths::HEAD)
        .expect("Unable to open HEAD file");

    writeln!(head_file, "{}", commit_id)
        .expect("Unable to write new commit_id to head");
}