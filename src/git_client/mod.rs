use crate::git_provider::SupportedProviders;

use self::git_cli::GitCli;
use eyre::Result;

pub mod git_cli;

pub enum CommitStatus {
    Success,
    Aborted,
}

#[derive(Debug)]
pub struct WorkingArea {
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
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

    /// Retrieves the body of the current commit.
    /// 
    /// # Returns
    /// 
    /// The body of the current commit as a `Result<String>`.
    fn get_current_commit_body(&self) -> Result<String>;

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

    /// Retrieves the working area of the current Git repository.
    ///
    /// # Returns
    ///
    /// The working area of the current Git repository as a `Result<WorkingArea>`.
    fn get_working_area(&self) -> Result<WorkingArea>;

    /// Creates a new blob with the specified content.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the blob.
    ///
    /// # Returns
    ///
    /// A `Result` containing the ID of the created blob on success, or an error on failure.
    fn create_blob(&self, content: &str) -> Result<String>;

    /// Reads the object with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `oid` - The ID of the object to read.
    ///
    /// # Returns
    ///
    /// A `Result` containing the content of the object on success, or an error on failure.
    fn read_object(&self, oid: &str) -> Result<String>;

    /// Updates the reference with the specified name to point to the specified object ID.
    ///
    /// # Arguments
    ///
    /// * `refname` - The name of the reference to update.
    /// * `oid` - The ID of the object to update the reference to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn update_ref(&self, refname: &str, oid: &str) -> Result<()>;

    /// Pushes the specified branch to the remote repository.
    ///
    /// # Arguments
    ///
    /// * `branch` - The name of the branch to push.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn push_branch(&self, branch: &String) -> Result<()>;
}

pub fn get_git_client() -> Result<Box<dyn GitClient>> {
    return Ok(Box::new(GitCli::new()?));
}
