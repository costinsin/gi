use super::GitClient;
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
    fn get_repository_info(&self) -> Result<(SupportedProviders, String, String)> {
        let out = Command::new("git")
            .args(["remote", "-v"])
            .output()
            .context("Failed to get git remote")
            .suggestion("Make sure you are inside a git repository")?
            .stdout
            .iter()
            .map(|&c| c as char)
            .collect::<String>();

        let is_https = match out.lines().next() {
            Some(line) => line.contains("https"),
            None => Err(eyre::eyre!("Unexpected git remote output"))
                .suggestion("Please check the connection to your remote repository")?,
        };

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

    fn get_repository_root(&self) -> Option<String> {
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
}
