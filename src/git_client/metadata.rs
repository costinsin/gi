use serde::{Deserialize, Serialize};

use super::GitClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchMetadata {
    #[serde(rename = "parentBranchName")]
    parent_branch_name: String,
    #[serde(rename = "parentBranchRevision")]
    parent_branch_revision: String,
}

impl BranchMetadata {
    pub fn new(parent_branch_name: String, parent_branch_revision: String) -> Self {
        Self {
            parent_branch_name,
            parent_branch_revision,
        }
    }
}

pub fn create_branch_metadata(
    git_client: &Box<dyn GitClient>,
    branch_name: String,
    metadata: &BranchMetadata,
) -> eyre::Result<()> {
    let metadata = serde_json::to_string(metadata)?;

    let object_sha = git_client.create_blob(&metadata)?;
    let ref_path = format!("refs/branch-metadata/{branch_name}");
    git_client.update_ref(&ref_path, &object_sha)?;

    Ok(())
}
