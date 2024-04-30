use eyre::{Ok, Result};

use crate::{
    git_client,
    git_provider::{provider_factory, GitProvider},
    project::settings::get_project_settings,
};

pub async fn submit() -> Result<()> {
    let (provider, owner, repo) = git_client::get_git_client()?.get_repo_info()?;
    let provider_obj = provider_factory(&provider)?;
    let trunk = get_project_settings()?.get_trunk()?;

    provider_obj
        .create_pull_request(&owner, &repo, &trunk)
        .await?;

    Ok(())
}
