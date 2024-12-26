use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::Write;
use toml;
use crate::structures::paths;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    index_hash: String,
    commit_message: String,
}

pub fn create_commit(commit_id: u64, index_hash: &str, commit_message: &str) {
    let new_commit = Commit {
        index_hash: index_hash.to_string(),
        commit_message: commit_message.to_string(),
    };

    let mut commits: HashMap<String, Commit> = read_commits().unwrap_or_else(|_| HashMap::new());
    let commit_key = format!("{}", commit_id);
    commits.insert(commit_key, new_commit);

    let toml_string = toml::to_string(&commits).expect("Failed to serialize commits");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(paths::COMMIT)
        .expect("Failed to open commit file");

    file.write_all(toml_string.as_bytes())
        .expect("Failed to write commits to file");
}

pub fn read_commits() -> Result<HashMap<String, Commit>, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(paths::COMMIT)?;
    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)?;

    let commits: HashMap<String, Commit> = toml::de::from_str(&toml_string)
        .expect("Failed to deserialize commits from TOML");

    Ok(commits)
}
