use color_eyre::Section;
use eyre::{Context, Result};

use super::GitProvider;

pub struct GitHub {}

impl GitHub {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl GitProvider for GitHub {
    async fn create_pull_request(
        &self,
        owner: &String,
        repo: &String,
        trunk: &String,
    ) -> Result<()> {
        println!("Creating pull request");

        let octocrab = octocrab::Octocrab::builder()
            .build()
            .context("Failed to create octocrab instance")
            .suggestion("Please check your GitHub credentials")?;

        let _pr = octocrab
            .pulls(owner, repo)
            .create("title", "head", trunk)
            .body("hello world!")
            .send()
            .await
            .context("Failed to create pull request")
            .suggestion("Please check your GitHub credentials")?;

        println!("Creating pull request");

        Ok(())
    }
}
