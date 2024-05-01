#![allow(async_fn_in_trait)]

use core::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use color_eyre::Section;
use eyre::Result;

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
        "github" => Ok(SupportedProviders::GitHub),
        p => Err(eyre::eyre!("Unsupported provider {}", p)).suggestion(format!(
            "Supported providers: {}.\n
            Add a remote that uses one of the currently supported providers.",
            SupportedProviders::get_providers()
        )),
    }
}

pub trait GitProvider {
    async fn create_pull_request(
        &self,
        owner: &String,
        repo: &String,
        trunk: &String,
    ) -> eyre::Result<()>;
}

pub fn provider_factory(provider: &SupportedProviders) -> Result<Box<impl GitProvider>> {
    match provider {
        SupportedProviders::GitHub => Ok(Box::new(github::GitHub::new()?)),
    }
}
