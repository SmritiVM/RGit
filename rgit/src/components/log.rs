use crate::structures::commit;
use crate::utils::message_handler::handle_message;

pub fn log_commits() {
    let commits = match commit::read_commits() {
        Ok(commits) => commits,
        Err(_) => {
            handle_message("Failed to read commits");
            return;
        }
    };
    for (commit_id, commit) in commits {
        log_commit(&commit_id, &commit);
    }
}

fn log_commit(commit_id: &str, commit: &commit::Commit) {
    handle_message(format!("Commit ID: {}", commit_id));
    handle_message(format!("Index Hash: {}", commit.index_hash));
    handle_message(format!("Commit Message: {}", commit.commit_message));
    handle_message("-----------------------------");
}
