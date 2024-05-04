use crate::git_provider::SupportedProviders;
use crate::IssueError;

use self::git_cli::GitCli;
use eyre::Result;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

pub mod git_cli;

pub enum CommitStatus {
    Success,
    Aborted,
}

/// The `GitClient` trait defines the interface for interacting with a Git repository.
pub trait GitClient: Send + Sync {
    /// Performs an interactive commit.
    ///
    /// # Returns
    ///
    /// A `Result` containing the commit status on success, or an error on failure.
    fn interactive_commit(&self) -> Result<CommitStatus>;

    /// Checks out the specified branch.
    ///
    /// # Arguments
    ///
    /// * `branch` - The name of the branch to check out.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn checkout(&self, branch: &str) -> Result<()>;

    /// Creates a new branch with the specified name.
    ///
    /// # Arguments
    ///
    /// * `branch` - The name of the new branch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the name of the created branch on success, or an error on failure.
    fn create_branch(&self, branch: &str) -> Result<String>;

    /// Retrieves information about the current Git repository.
    ///
    /// # Returns
    ///
    /// A `Result` containing a tuple with three elements: the provider, owner, and repository name.
    ///
    /// # Errors
    ///
    /// This function may return an error if:
    ///
    /// * The `git` command fails to execute.
    /// * The current directory is not a Git repository.
    fn get_repository_info(&self) -> Result<(SupportedProviders, String, String)>;

    /// Retrieves the root directory of the current Git repository.
    ///
    /// # Returns
    ///
    /// The root directory of the current Git repository as an `Option<String>`.
    fn get_repository_root(&self) -> Option<String>;

    /// Retrieves the name of the current branch.
    ///
    /// # Returns
    ///
    /// The name of the current branch as an `Option<String>`.
    fn get_current_branch(&self) -> Option<String>;

    /// Retrieves the title of the current commit.
    ///
    /// # Returns
    ///
    /// The title of the current commit as a `Result<String>`.
    fn get_current_commit_title(&self) -> Result<String>;

    /// Deletes the specified branch.
    ///
    /// # Arguments
    ///
    /// * `branch` - The name of the branch to delete.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn delete_branch(&self, branch: &str) -> Result<()>;
}

static GIT_CLIENT: Lazy<Mutex<Box<dyn GitClient>>> =
    Lazy::new(|| Mutex::new(Box::new(GitCli::new().unwrap())));

pub fn get_git_client() -> Result<MutexGuard<'static, Box<dyn GitClient>>> {
    let guard = GIT_CLIENT
        .lock()
        .to_issue_error("Failed to get git client lock")?;

    Ok(guard)
}
