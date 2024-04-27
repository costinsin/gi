use eyre::{Ok, Result};

use crate::{git_client::GitClient, project::settings::ProjectSettings};

pub fn create(git_client: &impl GitClient, project_settings: &mut ProjectSettings) -> Result<()> {
    Ok(())
}
