use cliclack::{
    log::{self, info},
    spinner,
};

use crate::{
    cli::StartArgs,
    connectors::{task::fetch_connector_task, utils::get_task_url_config_key},
    git::{GitBranch, GitConfig},
};

pub async fn start_task(params: &StartArgs) -> anyhow::Result<()> {
    info("Starting a new task")?;

    let mut spinner = spinner();

    spinner.start("Fetching the task...");

    let task_info = fetch_connector_task(params.link.as_str()).await?;

    spinner.stop(format!("Found task with summary: {}", task_info.name));

    let branch_name = format!("{}/{}", params.task_type, task_info.sanitized_name());

    if params.dry {
        log::info(format!("Simulate creating branch {}.", branch_name))?;
    } else {
        let config_key = get_task_url_config_key(&branch_name);

        GitConfig::write(&config_key, params.link.as_str())?;

        GitBranch::create(&branch_name)?;

        log::info(format!("Branch {} created!", branch_name))?;
    }

    Ok(())
}
