use cliclack::{
    log::{self, info},
    spinner,
};

use crate::{
    cli::StartArgs,
    git::{Git, GitConfig},
    task_connectors::task_connector::TaskConnector,
    utils::get_task_url_config_key,
};

pub async fn start_task(params: &StartArgs) -> anyhow::Result<()> {
    info("Starting a new task")?;

    let spinner = spinner();

    spinner.start("Fetching the task...");

    let connector = TaskConnector::from_url(&params.link.as_str())?;

    let task_info = connector.fetch_task().await?;

    spinner.stop(format!("Found task with summary: {}", task_info.name));

    let branch_name = format!("{}/{}", params.task_type, task_info.sanitized_name());

    if params.dry {
        log::info(format!("Simulate creating branch {}.", branch_name))?;
    } else {
        let config_key = get_task_url_config_key(&branch_name);

        GitConfig::write(&config_key, params.link.as_str())?;

        Git::switch(&branch_name, true)?;

        log::info(format!("Branch {} created!", branch_name))?;
    }

    Ok(())
}
