use anyhow::Context;
use cliclack::log;
use url::Url;

use crate::utils::get_current_task_url;

pub fn browse() -> anyhow::Result<()> {
    let task_url = get_current_task_url()?;

    Url::parse(&task_url).context("No task URL found. Start a task with 'mrburns start-task <url>' before running this command")?;

    log::info(format!("Opening the browser at {}", task_url))?;

    open::that(task_url.as_str())?;

    Ok(())
}
