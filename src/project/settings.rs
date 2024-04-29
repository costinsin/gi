use color_eyre::Section;
use dialoguer::theme::ColorfulTheme;
use eyre::{eyre, Ok, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Mutex};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProjectSettings {
    trunk: Option<String>,
}

pub static PROJECT_SETTINGS: Lazy<Mutex<ProjectSettings>> =
    Lazy::new(|| Mutex::new(ProjectSettings::load().unwrap()));

impl ProjectSettings {
    fn load() -> Result<Self> {
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

    pub fn set_trunk(&mut self, trunk: &String) -> Result<()> {
        self.trunk = Some(trunk.clone());
        self.save()?;

        Ok(())
    }

    pub fn get_trunk(&mut self) -> Result<String> {
        let trunk = match self.trunk.to_owned() {
            Some(a) => a,
            None => ask_for_trunk()?,
        };

        self.set_trunk(&trunk)?;

        Ok(trunk)
    }

    fn save(&self) -> Result<()> {
        let content = serde_json::to_string(self)?;
        std::fs::write(".git/.gi_project_config", content).map_err(|_| {
            eyre!("Failed to save project settings")
                .suggestion("Check if you have write permissions to the .git directory.")
        })?;

        Ok(())
    }
}

pub fn ask_for_trunk() -> Result<String> {
    let trunk = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("What is the name of the trunk branch?")
        .default("main".to_string())
        .interact()?;

    Ok(trunk)
}
