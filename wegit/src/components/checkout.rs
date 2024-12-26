write a function that simulates git checkout <commit id>
it should take <commit id> as a parameter.
there is a commit file stored in structures::paths::COMMIT
the checkout function should read all the commit objects by invoking the read_commits function in structures::commit

the function is as follows:
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

after reading the commits, it should fetch the index_hash corresponding to the commit_id in the parameter
after getting the index_hash, it should take objects dir as structures::paths::INDEX_OBJECTS and then call the retrieve objects function to get the contents of the index file at that point