#![allow(async_fn_in_trait)]

pub mod github;

pub trait GitProvider {
	async fn create_pull_request(&self, trunk: String) -> eyre::Result<()>;
}