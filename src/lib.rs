use color_eyre::Section;
use eyre::Result;
use std::sync::LockResult;

pub mod cli;
pub mod commands;
pub mod git_client;
pub mod project;
pub mod git_provider;

pub trait IssueError<T> {
    fn to_issue_error(self, error: &str) -> Result<T>;
}

impl<T> IssueError<T> for LockResult<T> {
    fn to_issue_error(self, error: &str) -> Result<T> {
        self.map_err(|_| {
            eyre::eyre!(error.to_owned()).suggestion("This is most likely a bug, please report it at https://github.com/costinsin/gi/issues")
        })
    }
}
