give the code to simulate git status
it has to display: changes to be committed and untracked files
for changes to be committted, compare index and staged. if staged is empty, no changes to be committed
if a filepath exists in staged but not in index, then it should come under new
if a filepath exists under staged and index but with different hashes, then it should be under modified

for untracked, traverse the entire current working directory and whatever is not in index or staged (except anything in .wegit) is considered
for reading index and status, the read_index function in structures::index can be used. it is as follows:

pub fn read_index(file_path: &str) -> std::io::Result<Index> {
    let mut index = Index::new();
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let filepath = parts[0].to_string();
            let hash_code = parts[1].to_string();
            index.add_index_object(&filepath, &hash_code);
        }
    }

    Ok(index)
}