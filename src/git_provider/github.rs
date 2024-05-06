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

use crate::{
    git_client,
    git_provider::{ask_for_pr_body, ask_for_pr_title},
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GitHub {}

impl GitHub {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl GitProvider for GitHub {
    fn ask_for_token(&self, path: &PathBuf) -> Result<String> {
        let token = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "Set up your GitHub personal access token. You can generate one at:
https://github.com/settings/tokens/new?description=gi&scopes=repo,read:org,read:user,user:email\n",
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
        let home_str = home_dir
            .to_str()
            .ok_or_eyre("Home directory isn't valid unicode")?;
        let token_file = [home_str, ".config", "gi", "token"]
            .iter()
            .collect::<PathBuf>();

        if !token_file.exists() {
            create_dir_all([home_str, ".config", "gi"].iter().collect::<PathBuf>())
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

            return self.ask_for_token(&token_file);
        }

        let data = fs::read_to_string(&token_file)
            .context("Failed to read file")
            .suggestion("Check if you have read permissions to the token file")?;
        let deserealized = serde_json::from_str::<serde_json::Value>(&data);

        match deserealized {
            Ok(value) => match value["accessToken"].as_str() {
                Some(token) => Ok(token.to_string()),
                None => self.ask_for_token(&token_file),
            },
            Err(_) => self.ask_for_token(&token_file),
        }
    }

    async fn create_pull_request(
        &self,
        owner: &String,
        repo: &String,
        branch: &String,
        trunk: &String,
    ) -> Result<()> {
        let token = self.get_token()?;

        let octocrab = octocrab::Octocrab::builder()
            .personal_token(token)
            .build()
            .context("Failed to create octocrab instance")
            .suggestion("Please check your GitHub personal access token")?;

        let git_client = git_client::get_git_client()?;

        let commit_title = git_client.get_current_commit_title()?;
        let title = ask_for_pr_title(&commit_title)?;

        let commit_body = git_client.get_current_commit_body()?;
        let body = ask_for_pr_body(&commit_body)?;

        let pr = octocrab
            .pulls(owner, repo)
            .create(title, branch, trunk)
            .body(body)
            .send()
            .await
            .context("Failed to create pull request")
            .suggestion("Please check your GitHub personal access token")?;

        let pr_url = pr.html_url.ok_or_eyre("Failed to get pull request URL")?;
        println!(
            "\nPull request created successfully! You can check it out at:\n{}",
            pr_url
        );

        Ok(())
    }
}
