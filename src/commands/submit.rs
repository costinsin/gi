use color_eyre::Section;
use eyre::{Context, Ok, OptionExt, Result};

use crate::{
    git_client,
    git_provider::{ask_for_pr_title, provider_factory, ask_for_pr_body, GitProvider},
    project::settings::get_project_settings,
};

pub async fn submit() -> Result<()> {
    let git_client = git_client::get_git_client()?;

    let branch = git_client
        .get_current_branch()
        .ok_or_eyre("Failed to get the current branch")
        .suggestion("Check whether you are checked out onto a branch")?;
    let trunk = get_project_settings()?.get_trunk()?;

    let commit_title = git_client
        .get_current_commit_title()
        .context("Failed to get the current commit title")
        .suggestion("Check if you have any commits in your branch")?;
    let title = ask_for_pr_title(&commit_title)?;

    let commit_body = git_client.get_current_commit_body()?;
    let body = ask_for_pr_body(&commit_body)?;    

    let (provider, owner, repo) = git_client.get_repository_info()?;
    let provider_obj = provider_factory(&provider)?;

    provider_obj
        .create_pull_request(&owner, &repo, &title, &branch, &trunk, &body)
        .await?;

    Ok(())
}
