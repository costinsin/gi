use color_eyre::Section;
use eyre::{Ok, OptionExt, Result};

use crate::{
    git_client,
    git_provider::{provider_factory, GitProvider},
    project::settings::get_project_settings,
};

pub async fn submit() -> Result<()> {
    let git_client = git_client::get_git_client()?;

    let branch = git_client
        .get_current_branch()
        .ok_or_eyre("Failed to get the current branch")
        .suggestion("Check whether you are checked out onto a branch")?;
    let trunk = get_project_settings()?.get_trunk()?;

    let (provider, owner, repo) = git_client.get_repository_info()?;
    let provider_obj = provider_factory(&provider)?;

    git_client.push_branch(&branch)?;
    provider_obj
        .create_pull_request(&owner, &repo, &branch, &trunk)
        .await?;

    Ok(())
}
