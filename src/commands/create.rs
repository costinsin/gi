use eyre::{Ok, Result};

use crate::project::settings::get_project_settings;

pub fn create() -> Result<()> {
    let trunk = get_project_settings()?.get_trunk()?;

    println!("Trunk: {}", trunk);

    Ok(())
}
