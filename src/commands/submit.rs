use color_eyre::Section;
use eyre::{Ok, Result};

use crate::{
    git_client::{self},
    git_provider::{self, GitProvider},
    project::settings::get_project_settings,
};

pub async fn submit() -> Result<()> {
    let (provider, _owner, _repo) = git_client::get_git_client()?.get_repo_info()?;

    let provider_obj = match provider.as_str() {
        "github" => git_provider::github::GitHub::new()?,
        _ => Err(eyre::eyre!("Unsupported provider")).suggestion("Supported providers: GitHub")?,
    };

    let trunk = get_project_settings()?.get_trunk()?;
    provider_obj.create_pull_request(trunk).await?;

    Ok(())
}
