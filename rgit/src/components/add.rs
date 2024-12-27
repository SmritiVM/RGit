use crate::structures::index::{Index}; 
use crate::utils::message_handler::handle_message;
use crate::utils::hash_and_compress;
use crate::structures::paths;
use std::path::Path;

pub fn add(filepath: &str) {
    let data = match std::fs::read(filepath) {
        Ok(data) => data,
        Err(e) => {
            handle_message(e);
            return
        },
    };
    let hash_code = hash_and_compress::calculate_sha1(&data); 

    write_objects(paths::INDEX, filepath, &hash_code);
    write_objects(paths::STAGED, filepath, &hash_code);

    handle_message(format!("File '{}' added.", filepath));
    hash_and_compress::create_object(Path::new(paths::FILE_OBJECTS), &data, &hash_code);
}

fn write_objects(filepath: &str, object_path: &str, object_hash: &str){
    let mut index = match Index::read_index(filepath) {
        Ok(index) => index, 
        Err(_) => Index::new(),
    };
    index.add_index_object(object_path, object_hash);
    if let Err(_) = index.write_index(filepath) {
        handle_message("Fatal, not a git repo!");
        return; 
    }
}