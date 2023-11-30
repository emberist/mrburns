use cliclack::{confirm, intro, log, spinner};
use std::{thread, time::Duration};
use url::Url;

use crate::{
    cli::MrArgs,
    config::Config,
    git::{GitBranch, GitConfig},
    gitlab::{add_merge_request_author, create_gitlab_mr},
    utils::{get_task_url_config_key, parse_repo_url, Domain, RepoUrlInfo},
};

fn fake_create_mr() -> anyhow::Result<()> {
    let git_remote_url =
        GitConfig::read("remote.origin.url").expect("Failed to retrieve git remote URL");

    let RepoUrlInfo { project, .. } =
        parse_repo_url(&git_remote_url).expect("Cannot parse repo url");

    log::info(format!("Skip creating the MR for project {}", project))?;

    let mut mr_spinner = spinner();

    mr_spinner.start("Simulating the MR creation...");

    thread::sleep(Duration::from_secs(2));

    mr_spinner.stop("MR created.");

    let mut author_spinner = spinner();

    author_spinner.start("Simulating MR author addition...");

    thread::sleep(Duration::from_secs(2));

    author_spinner.stop("Author added.");

    log::info("Simulation complete!")?;

    Ok(())
}

pub async fn create_mr(params: &MrArgs) -> anyhow::Result<()> {
    let current_branch_name = GitBranch::current().expect("Failed to retrieve current branch name");

    let task_url_config_key = get_task_url_config_key(&current_branch_name);

    let task_url = GitConfig::read(&task_url_config_key).expect("Failed to retrieve task URL");

    let issue_id = Url::parse(&task_url)
        .unwrap()
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .to_string();

    intro(format!("Creating MR for issue {}", issue_id))?;

    if issue_id.is_empty() {
        anyhow::bail!("No task id found.");
    }

    let target_branch = params
        .base_branch
        .to_owned()
        .unwrap_or(GitBranch::default()?);

    let confirmed = confirm(format!(
        "Creating MR for issue {}. {} <- {}",
        issue_id, target_branch, current_branch_name
    ))
    .interact()?;

    if !confirmed {
        return Ok(());
    }

    if params.dry {
        fake_create_mr()?;

        return Ok(());
    }

    let config = Config::read()?;

    if task_url.is_empty() {
        anyhow::bail!("No task URL found, consider creating the working branch using the `start-task` command");
    }

    let git_remote_url =
        GitConfig::read("remote.origin.url").expect("Failed to retrieve git remote URL");

    let RepoUrlInfo { domain, project } =
        parse_repo_url(&git_remote_url).expect("Cannot parse repo url");

    let create_response: crate::gitlab::CreateMrResponse = match domain {
        Domain::Gitlab => {
            let mut mr_spinner = spinner();

            mr_spinner.start("Creating the MR...");

            let create_response = create_gitlab_mr(
                &project,
                &task_url,
                &current_branch_name,
                &target_branch,
                config.create_draft_mr,
            )
            .await?;

            mr_spinner.stop("MR created.");

            let mut author_spinner = spinner();

            author_spinner.start("Adding MR author...");

            add_merge_request_author(&project, create_response.iid, create_response.author.id)
                .await?;

            author_spinner.stop("Author added.");

            create_response
        }
        _ => panic!("Not implemented yet"),
    };

    log::info(format!(
        "Opening it in the browser at {}",
        create_response.web_url
    ))?;

    open::that(create_response.web_url)?;

    Ok(())
}
