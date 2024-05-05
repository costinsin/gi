use std::{
    fs::{self, create_dir_all, File},
    path::PathBuf,
};

use color_eyre::Section;
use dialoguer::theme::ColorfulTheme;
use dirs;
use eyre::{Context, OptionExt, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::os::unix::fs::PermissionsExt;

use super::GitProvider;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GitHub {}

impl GitHub {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl GitProvider for GitHub {
    fn set_token(&self, path: &PathBuf) -> Result<String> {
        let token = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "Set up your GitHub personal access token. You can generate one at:
https://github.com/settings/tokens/new?description=Gi&scopes=repo,read:org,read:user,user:email\n",
            )
            .interact()?;

        let data = json!({
            "accessToken": token
        });

        fs::write(path, data.to_string())
            .context("Failed to write to token file")
            .suggestion("Check if you have write permissions")?;

        Ok(token)
    }

    fn get_token(&self) -> Result<String> {
        let home_dir = dirs::home_dir().ok_or_eyre("Failed to get home directory")?;
        let token_file = home_dir.join(".config").join("gi").join("token");

        if !token_file.exists() {
            create_dir_all(home_dir.join(".config").join("gi"))
                .context("Failed to create gi config directory")
                .suggestion("Check if you have write permissions to the .config directory")?;

            File::create_new(&token_file)
                .context("Failed to create token file")
                .suggestion("Check if you have write permissions to the .config/gi directory")?;

            /* TODO: Implement setting file permissions on Windows
             At the time of writing this code, fs::Permissions::from_mode only
             works on Unix systems. In the future, we'll find a way to perform
             the same operation on Windows. */

            let permissions = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&token_file, permissions)
                .context("Failed to set permissions on token file")?;

            return self.set_token(&token_file);
        }

        let data = fs::read_to_string(&token_file)
            .context("Failed to read file")
            .suggestion("Check if you have read permissions to the token file")?;
        let deserealized = serde_json::from_str::<serde_json::Value>(&data);

        match deserealized {
            Ok(value) => match value["accessToken"].as_str() {
                Some(token) => Ok(token.to_string()),
                None => self.set_token(&token_file),
            },
            Err(_) => self.set_token(&token_file),
        }
    }

    async fn create_pull_request(
        &self,
        owner: &String,
        repo: &String,
        title: &String,
        branch: &String,
        trunk: &String,
    ) -> Result<()> {
        let token = self.get_token()?;

        let octocrab = octocrab::Octocrab::builder()
            .personal_token(token)
            .build()
            .context("Failed to create octocrab instance")
            .suggestion("Please check your GitHub personal access token")?;

        let _pr = octocrab
            .pulls(owner, repo)
            .create(title, branch, trunk)
            .body("Automatically created by Gi")
            .send()
            .await
            .context("Failed to create pull request")
            .suggestion("Please check your GitHub personal access token")?;

        Ok(())
    }
}
