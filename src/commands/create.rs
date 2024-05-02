use color_eyre::Section;
use eyre::{Context, Ok, Result};
use regex::Regex;

use crate::git_client::get_git_client;

pub fn create() -> Result<()> {
    let git_client = get_git_client()?;

    let Some(base_branch) = git_client.get_current_branch() else {
        return Err(
            eyre::eyre!("Can't create a new stacked commit without a current branch.")
                .suggestion("Checkout onto a branch and try again."),
        );
    };
    let temp_branch = git_client.create_branch("gi_temp_branch")?;
    git_client.checkout(&temp_branch)?;

    let commit_status = git_client.interactive_commit()?;
    match commit_status {
        crate::git_client::CommitStatus::Aborted => {
            git_client.checkout(&base_branch)?;
            git_client.delete_branch(&temp_branch)?;
            return Ok(());
        }
        crate::git_client::CommitStatus::Success => {}
    }

    let commit_title = git_client.get_current_commit_title()?;
    let new_branch = git_client.create_branch(&format_commit_title(commit_title)?)?;
    git_client.checkout(&new_branch)?;

    git_client.delete_branch(&temp_branch)?;

    Ok(())
}

/// Formats the commit title by replacing non-alphanumeric characters with underscores and prepending the current date.
///
/// # Arguments
///
/// * `title` - The original commit title.
///
/// # Errors
///
/// This function can return an error if the regular expression fails to create a regex pattern.
///
/// # Examples
///
/// `format_commit_title("Hello, World")` returns `Ok("12-31_Hello_World")`
fn format_commit_title(title: String) -> Result<String> {
    // Get day and month from the current date
    let today = chrono::Local::now().format("%m-%d").to_string();

    let non_alphanumeric = Regex::new(r"[\W_]+").context("Failed to create regex")?;
    let normalized_title = non_alphanumeric.replace_all(&title, "_").to_string();

    // Format the commit title
    Ok(format!("{}-{}", today, normalized_title))
}
