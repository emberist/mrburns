use cliclack::{
    confirm,
    log::{self},
    spinner,
};

use std::{thread, time::Duration};

use crate::{
    cli::MrArgs,
    connectors::{
        models::Mergeble,
        task::fetch_connector_task,
        utils::{get_task_url_config_key, parse_repo_connector_url, parse_task_connector_url},
    },
    git::{GitBranch, GitConfig},
};

fn fake_create_mr() -> anyhow::Result<()> {
    let mr_spinner = spinner();

    mr_spinner.start("Simulating the MR creation...");

    thread::sleep(Duration::from_secs(2));

    mr_spinner.stop("MR created.");

    let author_spinner = spinner();

    author_spinner.start("Simulating MR author addition...");

    thread::sleep(Duration::from_secs(2));

    author_spinner.stop("Author added.");

    log::info("Simulation complete!")?;

    Ok(())
}

pub async fn create_mr(params: &MrArgs) -> anyhow::Result<()> {
    let current_branch_name = GitBranch::current()?;

    let task_url_config_key = get_task_url_config_key(&current_branch_name);

    let task_url = GitConfig::read(&task_url_config_key)?;

    parse_task_connector_url(&task_url)?;

    let target_branch = params
        .base_branch
        .to_owned()
        .unwrap_or(GitBranch::default()?);

    let confirmed = confirm(format!(
        "Creating MR: {} <- {}",
        target_branch, current_branch_name
    ))
    .interact()?;

    if !confirmed {
        return Ok(());
    }

    if params.dry {
        fake_create_mr()?;

        return Ok(());
    }

    if task_url.is_empty() {
        anyhow::bail!("No task URL found, consider creating the working branch using the `start-task` command");
    }

    let mr_spinner = spinner();

    mr_spinner.start("Getting task informations...");

    let task_info = fetch_connector_task(&task_url).await?;

    mr_spinner.stop(format!("Task {} found.", task_info.name));

    let git_remote_url = GitConfig::read("remote.origin.url")?;

    let connector = parse_repo_connector_url(&git_remote_url).expect("Cannot parse repo url");

    let url = connector.mr_url(&task_info, &target_branch)?;

    log::info(format!("Opening: {}", url))?;

    open::that(url)?;

    Ok(())
}
