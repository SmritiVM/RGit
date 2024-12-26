use crate::utils::hash_and_compress;
use crate::structures::commit;
use crate::structures::paths;
use crate::utils::message_handler::handle_message;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn commit_changes(commit_message: &str) {
    let staged_path = Path::new(paths::STAGED);
    if !staged_path.exists() {
        handle_message("No changes staged for commit.");
        return; 
    }

    let commit_id = match get_current_commit_id() {
        Some(id) => id + 1,
        None => {
            handle_message("Failed to get commit ID.");
            return
        }
    };

    update_head(commit_id);

    let index_data = match std::fs::read(paths::INDEX){
        Ok(index_data) => index_data,
        Err(e) => {
            handle_message(e);
            return; 
        }
    };

    let index_hash = hash_and_compress::calculate_sha1(&index_data); 
    commit::create_commit(commit_id, &index_hash, commit_message);
    hash_and_compress::create_object(Path::new(paths::INDEX_OBJECTS), &index_data, &index_hash)

    delete_staged_changes(staged_path);
}

fn get_current_commit_id() -> Option<u64> {
    let mut index_content = String::new();  
    let commit_id: u64 = match File::open(paths::HEAD)
        .and_then(|mut file| file.read_to_string(&mut index_content))  
    {
        Ok(_) => match index_content.trim().parse::<u64>() {
            Ok(id) => id,  
            Err(e) => {
                handle_message(e);
                return None; 
            }
        },
        Err(e) => {
            handle_message(e);
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

fn delete_staged_changes(staged_path) {
    if let Err(e) = remove_dir_all(staged_path) {
        handle_message(format!("Failed to delete staged changes: {}", e));
    } 
}