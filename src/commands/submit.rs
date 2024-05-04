use color_eyre::Section;
use eyre::{Ok, OptionExt, Result};

use crate::{
    git_client, git_provider::provider_factory, git_provider::GitProvider,
    project::settings::get_project_settings,
};

pub async fn submit() -> Result<()> {
    let git_client = git_client::get_git_client()?;

    let branch = git_client
        .get_current_branch()
        .ok_or_eyre("Failed to get the current branch")
        .suggestion("Check whether you are checked out onto a branch")?;
    let title = branch.clone();
    let (provider, owner, repo) = git_client.get_repository_info()?;

    drop(git_client);

    let trunk = get_project_settings()?.get_trunk()?;
    let provider_obj = provider_factory(&provider)?;

    provider_obj
        .create_pull_request(&owner, &repo, &title, &branch, &trunk)
        .await?;

    Ok(())
}
