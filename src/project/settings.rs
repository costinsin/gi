use std::path::Path;

use color_eyre::Section;
use eyre::{eyre, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProjectSettings {
    pub trunk: Option<String>,
}

impl ProjectSettings {
    pub fn new() -> Result<Self> {
        if !Path::new(".git").exists() {
            return Err(eyre!("You are not inside a git repository.").suggestion(
                "Run `gt` inside a git repository or run `git init` to create a new one.",
            ));
        }

        // Check if the project has a .gi_project_config file, if not, return a default ProjectSettings
        let config_path = Path::new(".git/.gi_project_config");
        if !config_path.exists() {
            return Ok(Self::default());
        }

        // Read the file, if there is an error, return a default ProjectSettings
        let content = std::fs::read_to_string(config_path);
        if content.is_err() {
            return Ok(Self::default());
        }

        // Parse the content, if there is an error, return a default ProjectSettings
        let settings = serde_json::from_str::<ProjectSettings>(&content?);
        if settings.is_err() {
            return Ok(Self::default());
        }

        Ok(settings?)
    }

    pub fn set_trunk(&mut self, trunk: String) -> &mut Self {
        self.trunk = Some(trunk);
        self
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string(self)?;
        std::fs::write(".git/.gi_project_config", content).map_err(|_| {
            eyre!("Failed to save project settings")
                .suggestion("Check if you have write permissions to the .git directory.")
        })?;

        Ok(())
    }
}
