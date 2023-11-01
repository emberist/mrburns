use anyhow::bail;

use crate::{
    config::Config,
    git::{GitBranch, GitConfig},
    github::api::open_github_pull_request_creation_url,
    gitlab::api::create_gitlab_mr,
};

use super::{
    models::{RepoInfo, RepoProvider, TaskInfo},
    utils::parse_repo_url,
};

pub async fn create_merge_request(task_info: &TaskInfo) -> anyhow::Result<String> {
    let git_remote_url = GitConfig::read("remote.origin.url")?;

    let RepoInfo { provider, project } =
        parse_repo_url(&git_remote_url).expect("Cannot parse repo url");

    let url = match provider {
        RepoProvider::Gitlab => {
            let current_branch_name = GitBranch::current()?;
            let config = Config::read()?;

            create_gitlab_mr(
                &project,
                task_info,
                &current_branch_name,
                &config.base_branch,
                config.create_draft_mr,
            )
            .await?
            .web_url
        }
        RepoProvider::Github => open_github_pull_request_creation_url(&project, task_info)?,
        RepoProvider::Bitbucket => bail!("Bitbucket is not supported yet."),
    };

    Ok(url)
}
