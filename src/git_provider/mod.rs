#![allow(async_fn_in_trait)]

use core::fmt;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use color_eyre::Section;
use eyre::{Context, OptionExt, Result};

pub mod github;

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
    pub fn get_providers() -> String {
        SupportedProviders::iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

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

pub fn ask_for_pr_title(commit_title: &String) -> Result<String> {
    let title = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Set the PR title:")
        .default(commit_title.to_owned())
        .interact()?;

    Ok(title)
}

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

pub trait GitProvider {
    fn set_token(&self, path: &PathBuf) -> Result<String>;
    fn get_token(&self) -> Result<String>;

    async fn create_pull_request(
        &self,
        owner: &String,
        repo: &String,
        title: &String,
        branch: &String,
        trunk: &String,
        body: &String,
    ) -> eyre::Result<()>;
}

pub fn provider_factory(provider: &SupportedProviders) -> Result<Box<impl GitProvider>> {
    match provider {
        SupportedProviders::GitHub => Ok(Box::new(github::GitHub::new()?)),
    }
}
