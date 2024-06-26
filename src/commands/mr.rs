use anyhow::Context;
use cliclack::{
    confirm,
    log::{self},
    spinner,
};

use crate::{
    cli::MrArgs,
    git::{adapter::GitClientAdapter, client::GitClient},
    repo_connectors::models::RepoConnector,
    task_connectors::task_connector::TaskConnector,
};

pub async fn create_mr(params: &MrArgs) -> anyhow::Result<()> {
    let git = GitClient {};

    let current_branch_name = git.current_branch()?;

    let task_connector = TaskConnector::from_task_url(&git)?;

    let target_branch = params
        .base_branch
        .to_owned()
        .unwrap_or(git.default_branch()?);

    let confirmed = confirm(format!(
        "Creating MR: {} <- {}",
        target_branch, current_branch_name
    ))
    .interact()?;

    if !confirmed {
        return Ok(());
    }

    let mr_spinner = spinner();

    mr_spinner.start("Getting task informations...");

    let task_info = task_connector.fetch_task().await?;

    mr_spinner.stop(format!("Task {} found.", task_info.name));

    let git_remote_url = git.read_config("remote.origin.url")?;

    let repo_connector =
        RepoConnector::from_remote(&git_remote_url).context("Cannot parse repo url")?;

    let url = repo_connector.create_mr_url(&task_info, &target_branch)?;

    log::info(format!("Opening: {}", url))?;

    open::that(url)?;

    Ok(())
}
