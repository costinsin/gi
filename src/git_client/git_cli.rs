use super::{CommitStatus, GitClient};
use crate::git_provider::{get_provider_enum, SupportedProviders};
use color_eyre::Section;
use eyre::{Context, ContextCompat, Result};
use regex::Regex;
use std::process::Command;

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
    fn interactive_commit(&self) -> Result<CommitStatus> {
        // Executes the `git commit -s` command to perform an interactive commit.
        let status = Command::new("git")
            .args(["commit", "-s"])
            .status()
            .context("Interactive commit failed")?;

        if !status.success() {
            return Ok(CommitStatus::Aborted);
        }

        Ok(CommitStatus::Success)
    }

    /// Checks out the specified branch.
    ///
    /// # Arguments
    ///
    /// * `branch` - The name of the branch to check out.
    fn checkout(&self, branch: &str) -> Result<()> {
        // Executes the `git checkout <branch>` command to check out the specified branch.
        let output = Command::new("git")
            .args(["checkout", branch])
            .output()
            .context("Failed to checkout branch")?;

        if !output.status.success() {
            return Err(eyre::eyre!("Failed to checkout branch"));
        }

        Ok(())
    }

    /// Creates a new branch with the specified name.
    ///
    /// # Arguments
    ///
    /// * `branch` - The name of the new branch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the name of the created branch on success, or an error on failure.
    fn create_branch(&self, branch: &str) -> Result<String> {
        // Executes the `git branch <branch>` command to create a new branch.
        let output = Command::new("git").args(["branch", branch]).output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(branch.to_string())
                } else {
                    // If the branch creation fails, appends a random string to the branch name and recursively calls `create_branch` again.
                    let random_string = rand::random::<u8>().to_string();

                    let new_branch = branch.to_string() + &random_string;
                    Ok(self.create_branch(&new_branch)?)
                }
            }
            Err(e) => Err(e).context("Failed to create a new branch"),
        }
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
    fn get_repository_info(&self) -> Result<(SupportedProviders, String, String)> {
        // Executes the `git config --get remote.origin.url` command to get the URL of the remote repository.
        let out = Command::new("git")
            .args(["config", "--get", "remote.origin.url"])
            .output()
            .context("Failed to get git remote")
            .suggestion("Make sure you are inside a git repository")?
            .stdout
            .iter()
            .map(|&c| c as char)
            .collect::<String>();

        let is_https = out.starts_with("https");

        // Parses the remote repository URL to extract the provider, owner, and repository name.
        let regex = if is_https {
            Regex::new(r"https://(?P<provider>[^\.]+).*/(?P<owner>.+)/(?P<repo>.+).git").unwrap()
        } else {
            Regex::new(r"@(?P<provider>[^\.]+).*:(?P<owner>.+)/(?P<repo>.+).git").unwrap()
        };

        let captures = regex
            .captures(&out)
            .context("Unrecognised git repository URL format")
            .suggestion("Use a HTTPS or SSH git repository URL")?;

        let provider = get_provider_enum(captures.name("provider").unwrap().as_str())?;
        let owner = captures.name("owner").unwrap().as_str().to_string();
        let repo = captures.name("repo").unwrap().as_str().to_string();

        Ok((provider, owner, repo))
    }

    /// Retrieves the root directory of the current Git repository.
    ///
    /// # Returns
    ///
    /// The root directory of the current Git repository as an `Option<String>`.
    fn get_repository_root(&self) -> Option<String> {
        // Executes the `git rev-parse --show-toplevel` command to get the root directory of the current Git repository.
        let root = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output();

        match root {
            Ok(output) => Some(
                output
                    .stdout
                    .iter()
                    .map(|&c| c as char)
                    .collect::<String>()
                    .trim()
                    .to_string(),
            ),
            Err(_) => None,
        }
    }

    /// Retrieves the name of the current branch.
    ///
    /// # Returns
    ///
    /// The name of the current branch as an `Option<String>`.
    fn get_current_branch(&self) -> Option<String> {
        // Executes the `git symbolic-ref -q --short HEAD` command to get the name of the current branch.
        let branch = Command::new("git")
            .args(["symbolic-ref", "-q", "--short", "HEAD"])
            .output();

        match branch {
            Ok(output) => match output.status.code() {
                Some(0) => String::from_utf8(output.stdout)
                    .map(|s| s.trim().to_string())
                    .ok(),
                _ => None,
            },
            Err(_) => None,
        }
    }

    /// Retrieves the title of the current commit.
    ///
    /// # Returns
    ///
    /// The title of the current commit as a `Result<String>`.
    fn get_current_commit_title(&self) -> Result<String> {
        // Executes the `git log -1 --pretty=%s` command to get the title of the current commit.
        let output = Command::new("git")
            .args(["log", "-1", "--pretty=%s"])
            .output()
            .context("Failed to get the current commit title")?;

        let title = String::from_utf8(output.stdout)
            .context("Failed to parse the current commit title")?
            .trim()
            .to_string();

        Ok(title)
    }

    /// Deletes the specified branch.
    ///
    /// # Arguments
    ///
    /// * `branch` - The name of the branch to delete.
    fn delete_branch(&self, branch: &str) -> Result<()> {
        // Executes the `git branch -D <branch>` command to delete the specified branch.
        let output = Command::new("git")
            .args(["branch", "-D", branch])
            .output()
            .context("Failed to delete branch")?;

        if !output.status.success() {
            return Err(eyre::eyre!("Failed to delete branch"));
        }

        Ok(())
    }
}
