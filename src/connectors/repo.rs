use crate::git::GitConfig;

use super::{
    models::{Mergeble, TaskDetails},
    utils::parse_repo_connector_url,
};

pub fn create_merge_request_creation_url(task_info: &TaskDetails) -> anyhow::Result<String> {
    let git_remote_url = GitConfig::read("remote.origin.url")?;

    let connector = parse_repo_connector_url(&git_remote_url).expect("Cannot parse repo url");

    connector.mr_url(task_info)
}
