use std::process;

use color_eyre::Section;
use eyre::{Context, Result};

use super::GitClient;

pub struct GitCli {}

impl GitCli {
    pub fn new() -> Result<Self> {
        // Check if git is installed
        which::which("git").map_err(|_| {
            eyre::eyre!("Git is not installed").suggestion(
                "Install git: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git",
            )
        })?;

        Ok(Self {})
    }
}

/// Implementation of the `GitClient` trait for the `GitCli` struct.
impl GitClient for GitCli {
    /// Performs an interactive commit.
    fn interactive_commit(&self) {
        println!("Committing changes");
    }

    /// Checks out the specified branch.
    ///
    /// # Arguments
    ///
    /// * `branch` - The name of the branch to check out.
    fn checkout(&self, branch: &str) {
        println!("Checking out branch: {}", branch);
    }

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
    fn get_repo_info(&self) -> Result<(String, String, String)> {
        let out = process::Command::new("git")
            .arg("remote")
            .arg("-v")
            .output()
            .context("Failed to get git remote")
            .suggestion("Make sure you are inside a git repository")?
            .stdout
            .iter()
            .map(|&c| c as char)
            .collect::<String>();

        let mut parts = out
            .split(['/', '@', ':', '.'])
            .collect::<Vec<&str>>();
        parts.retain(|&s| !s.is_empty());

        let provider = parts[1].to_string();
        let owner = parts[3].to_string();
        let repo = parts[4].to_string();

        Ok((provider, owner, repo))
    }
}
