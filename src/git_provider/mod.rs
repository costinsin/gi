#![allow(async_fn_in_trait)]

use core::fmt;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use color_eyre::Section;
use eyre::{Context, OptionExt, Result};

pub mod github;

/// Enum representing the supported Git providers.
#[derive(EnumIter)]
pub enum SupportedProviders {
    GitHub,
}

impl fmt::Display for SupportedProviders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SupportedProviders::GitHub => write!(f, "GitHub"),
        }
    }
}

impl SupportedProviders {
    /// Returns a string containing the names of all supported providers.
    pub fn get_providers() -> String {
        SupportedProviders::iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

/// Converts a string representation of a provider to the corresponding `SupportedProviders` enum variant.
///
/// # Arguments
///
/// * `provider` - The string representation of the provider.
///
/// # Returns
///
/// Returns a `Result` containing the corresponding `SupportedProviders` enum variant if successful, or an error if the provider is unsupported.
pub fn get_provider_enum(provider: &str) -> Result<SupportedProviders> {
    match provider {
        p if p.starts_with("github") => Ok(SupportedProviders::GitHub),
        p => Err(eyre::eyre!("Unsupported provider {}", p)).suggestion(format!(
            "Supported providers: {}.\n
            Add a remote that uses one of the currently supported providers.",
            SupportedProviders::get_providers()
        )),
    }
}

/// Asks the user to input the title for a pull request.
///
/// # Arguments
///
/// * `commit_title` - The default title to display in the input prompt.
///
/// # Returns
///
/// Returns a `Result` containing the user-provided title as a `String` if successful, or an error if the input prompt fails.
pub fn ask_for_pr_title(commit_title: &String) -> Result<String> {
    let title = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Set the PR title:")
        .default(commit_title.to_owned())
        .interact()?;

    Ok(title)
}

/// Asks the user to input the body for a pull request.
///
/// # Arguments
///
/// * `commit_body` - The default body to display in the input prompt.
///
/// # Returns
///
/// Returns a `Result` containing the user-provided body as a `String` if successful, or an error if the input prompt fails.
pub fn ask_for_pr_body(commit_body: &String) -> Result<String> {
    let options = vec![
        "Use commit body",
        "Edit commit body",
        "Custom description",
        "No description",
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Set the PR description:")
        .default(0)
        .items(&options)
        .interact()?;

    let body = match options[selection] {
        "Use commit body" => commit_body.to_owned(),
        "Edit commit body" => dialoguer::Editor::new()
            .edit(commit_body)
            .context("Failed to open the default editor")?
            .ok_or_eyre("The editor was closed without saving")?,
        "Custom description" => dialoguer::Editor::new()
            .edit("")
            .context("Failed to open the default editor")?
            .ok_or_eyre("The editor was closed without saving")?,
        _ => String::new(),
    };

    Ok(body)
}

/// Trait representing a Git provider.
pub trait GitProvider {
    /// Sets the authentication token.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file where the token should be stored.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the token as a `String` if successful, or an error if the token cannot be set.
    fn ask_for_token(&self, path: &PathBuf) -> Result<String>;

    /// Retrieves the authentication token.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the token as a `String` if successful, or an error if the token cannot be retrieved.
    fn get_token(&self) -> Result<String>;

    /// Creates a pull request.
    ///
    /// # Arguments
    ///
    /// * `owner` - The owner of the repository.
    /// * `repo` - The name of the repository.
    /// * `branch` - The name of the branch to create the pull request from.
    /// * `trunk` - The name of the trunk branch to create the pull request against.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `()` if successful, or an error if the pull request cannot be created.
    async fn create_pull_request(
        &self,
        owner: &String,
        repo: &String,
        branch: &String,
        trunk: &String,
    ) -> eyre::Result<()>;
}

/// Creates an instance of the Git provider based on the specified `SupportedProviders` enum variant.
///
/// # Arguments
///
/// * `provider` - The `SupportedProviders` enum variant representing the desired Git provider.
///
/// # Returns
///
/// Returns a `Result` containing a boxed trait object implementing the `GitProvider` trait if successful, or an error if the provider fails to initialize.
pub fn provider_factory(provider: &SupportedProviders) -> Result<Box<impl GitProvider>> {
    match provider {
        SupportedProviders::GitHub => Ok(Box::new(github::GitHub::new()?)),
    }
}
