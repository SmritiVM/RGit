use crate::components::hash_and_compress;
use crate::structures::commit;
use crate::structures::paths;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Error, ErrorKind};

#[derive(Debug, Clone)]
struct Commit {
    commit_id: u64,
    index_hash: String,
    commit_message: String,
}

pub fn commit_changes(commit_message: &str) {
    let commit_id = match get_current_commit_id() {
        Some(id) => id + 1,
        None => {
            eprintln!("Failed to get commit ID.");
            return
        }
    };

    let index_hash = match File::open(paths::INDEX)
    .and_then(|mut file| file.read_to_string(&mut String::new()).map(|_| file))
    {
        Ok(_) => hash_and_compress::calculate_sha1(String::new().as_bytes()),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    commit::create_commit(&commit_id, &index_hash, commit_message);
}

fn get_current_commit_id() -> Option<u64>{
    let commit_id: u64 = match File::open(paths::HEAD)
    .and_then(|mut file| file.read_to_string(&mut String::new()).map(|_| file))
    .and_then(|_| String::new().trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
    {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error: {}", e);
            return None;
        }
    };
    Some(commit_id)
}



