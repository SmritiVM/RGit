use crate::structures::paths;
use std::fs::{OpenOptions};
use std::io::{Write};

#[derive(Debug, Clone)]
struct Commit {
    commit_id: u64,
    index_hash: String,
    commit_message: String,
}

pub fn create_commit(commit_id: &u64, index_hash: &str, commit_message: &str) {
    let new_commit = Commit {
        commit_id: *commit_id,
        index_hash: index_hash.to_string(),
        commit_message: commit_message.to_string(),
    };

    if let Err(e) = OpenOptions::new()
    .append(true)
    .create(true)
    .open(paths::COMMIT)
    .and_then(|mut file| writeln!(file, "{:?}", new_commit).map(|_| file))
    {
        eprintln!("Error: {}", e);
        return;
    }
}
