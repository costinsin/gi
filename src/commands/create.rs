use crate::git_client::{self};

pub fn create(git_client: &impl git_client::GitClient) {
    git_client.interactive_commit();
    git_client.checkout("main")
}
