use eyre::{Ok, Result};

use crate::{project::settings::PROJECT_SETTINGS, IssueError};

pub fn create() -> Result<()> {
    let trunk = PROJECT_SETTINGS
        .lock()
        .to_issue_error("Failed to get project settings mutex")?
        .get_trunk()?;

    println!("Trunk: {}", trunk);

    Ok(())
}
