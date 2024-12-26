use std::fs::{self, OpenOptions};
use std::io::{Write};
use std::path::Path;
use std::env;
use crate::structures::commit::read_commits;
use crate::structures::paths::{INDEX_OBJECTS, FILE_OBJECTS}; 
use crate::utils::hash_and_compress;

pub fn checkout(commit_id: &str) {
    let working_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return, 
    };

    let commits = match read_commits() {
        Ok(commits) => commits,
        Err(_) => return, 
    };

    if let Some(commit) = commits.get(commit_id) {
        let index_hash = &commit.index_hash;
        let index_data = match hash_and_compress::retrieve_object(Path::new(INDEX_OBJECTS), index_hash) {
            Ok(data) => data,
            Err(_) => return, 
        };

        let index_file_content = match String::from_utf8(index_data) {
            Ok(content) => content,
            Err(_) => return, 
        };
        let lines = index_file_content.lines();

        for line in lines {
            let mut parts = line.split_whitespace();
            if let (Some(file_path), Some(file_content_hash)) = (parts.next(), parts.next()) {
                let file_content = match hash_and_compress::retrieve_object(Path::new(FILE_OBJECTS), file_content_hash) {
                    Ok(content) => content,
                    Err(_) => continue, 
                };

                let full_path = working_dir.join(file_path);
                if let Err(_) = fs::create_dir_all(full_path.parent().unwrap()) {
                    continue;
                }

                if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(full_path) {
                    let _ = file.write_all(&file_content);
                }
            }
        }
    }
}