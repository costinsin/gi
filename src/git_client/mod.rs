pub mod git_cli;

pub trait GitClient {
    fn interactive_commit(&self);
    fn checkout(&self, branch: &str);
}
