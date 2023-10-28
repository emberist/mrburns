use cliclack::log;
use url::Url;

use crate::{
    git::{GitBranch, GitConfig},
    utils::get_task_url_config_key,
};

pub fn browse() -> anyhow::Result<()> {
    let current_branch_name = GitBranch::current()?;

    let task_url_config_key = get_task_url_config_key(&current_branch_name);

    let task_url = GitConfig::read(&task_url_config_key)?;

    Url::parse(&task_url).unwrap_or_else(|_| {
        log::error("No task URL found. Start a task with 'mrburns start-task <url>' before running this command").unwrap();

        std::process::exit(0);
    });

    log::info(format!("Opening the browser at {}", task_url))?;

    open::that(task_url.as_str())?;

    Ok(())
}