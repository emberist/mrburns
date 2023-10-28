use cliclack::{intro, log, spinner};

use crate::{
    cli::StartArgs,
    git::{GitBranch, GitConfig},
    jira::fetch_jira_task_by_issue_id,
    strings::slugify::slugify,
    utils::get_task_url_config_key,
};

pub async fn start_task(params: &StartArgs) -> anyhow::Result<()> {
    let issue_id = params
        .link
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .to_string();

    intro(format!("Starting a new task based on issue {}", issue_id))?;

    let mut spinner = spinner();

    spinner.start("Fetching the task...");

    let issue = fetch_jira_task_by_issue_id(&issue_id)
        .await
        .expect("Error fetching the task");

    spinner.stop(format!(
        "Found task with summary: {}\n",
        issue.fields.summary
    ));

    let normalized_summary = slugify(&issue.fields.summary);

    let branch_name = format!("{}/{}-{}", params.task_type, issue.key, normalized_summary);

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
