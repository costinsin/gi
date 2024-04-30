use crate::IssueError;
use crate::git_provider::SupportedProviders;

use self::git_cli::GitCli;
use eyre::Result;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

pub mod git_cli;

pub trait GitClient: Send + Sync {
    fn interactive_commit(&self);
    fn checkout(&self, branch: &str);
    fn get_repo_info(&self) -> Result<(SupportedProviders, String, String)>;
    fn get_repository_root(&self) -> Option<String>;
}

static GIT_CLIENT: Lazy<Mutex<Box<dyn GitClient>>> =
    Lazy::new(|| Mutex::new(Box::new(GitCli::new().unwrap())));

pub fn get_git_client() -> Result<MutexGuard<'static, Box<dyn GitClient>>> {
    let guard = GIT_CLIENT
        .lock()
        .to_issue_error("Failed to get git client lock")?;

    Ok(guard)
}

