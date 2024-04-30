use std::process::Command;

use color_eyre::Section;
use eyre::Result;

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

impl GitClient for GitCli {
    fn interactive_commit(&self) {
        println!("Committing changes");
    }

    fn checkout(&self, branch: &str) {
        println!("Checking out branch: {}", branch);
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
