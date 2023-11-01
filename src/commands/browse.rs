use anyhow::Context;
use cliclack::log;
use url::Url;

use crate::{
    connectors::utils::get_task_url_config_key,
    git::{GitBranch, GitConfig},
};

pub fn browse() -> anyhow::Result<()> {
    let current_branch_name = GitBranch::current()?;

    let task_url_config_key = get_task_url_config_key(&current_branch_name);

    let task_url = GitConfig::read(&task_url_config_key)?;

    Url::parse(&task_url).context("No task URL found. Start a task with 'mrburns start-task <url>' before running this command")?;

    log::info(format!("Opening the browser at {}", task_url))?;

    open::that(task_url.as_str())?;

    Ok(())
}
