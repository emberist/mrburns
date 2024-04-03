use anyhow::Context;
use regex::Regex;

use crate::{
    asana::utils::{get_asana_task_id_from_url, get_asana_task_url_regex},
    github::utils::{get_github_issue_info_from_url, get_github_issue_url_regex},
    jira::utils::{get_jira_task_info_from_url, get_jira_task_url_regex},
};

use super::models::{RepoConnector, TaskConnector};

pub fn parse_task_connector_url(url: &str) -> anyhow::Result<TaskConnector> {
    // TODO: use enum matches

    if get_asana_task_url_regex().is_match(url) {
        let task_id = get_asana_task_id_from_url(url)?;

        return Ok(TaskConnector::Asana(task_id.to_string()));
    }

    if get_jira_task_url_regex().is_match(url) {
        let (api_base_url, task_id) = get_jira_task_info_from_url(url)?;

        return Ok(TaskConnector::Jira {
            api_base_url: api_base_url.to_string(),
            task_id: task_id.to_string(),
        });
    }

    if get_github_issue_url_regex().is_match(url) {
        let (repo, issue_id) = get_github_issue_info_from_url(url)?;

        return Ok(TaskConnector::Github {
            repo: repo.to_string(),
            issue_id,
        });
    }

    anyhow::bail!("Url does not match any connectors")
}

pub fn parse_repo_connector_url(url: &str) -> anyhow::Result<RepoConnector> {
    let captures = Regex::new(
        r"^(?:git@|https:\/\/)(gitlab\.com|github\.com|bitbucket\.org)(?::|\/)(.+)\.git$",
    )?
    .captures(url)
    .ok_or_else(|| anyhow::anyhow!("No regex match found"))?;

    let url_domain = captures.get(1).expect("No  match found").as_str();

    let project = captures
        .get(2)
        .context("No project id found")?
        .as_str()
        .to_string();

    let connector = match url_domain {
        "github.com" => RepoConnector::Github(project),
        "gitlab.com" => RepoConnector::Gitlab(project),
        "bitbucket.org" => RepoConnector::Bitbucket(project),
        _ => anyhow::bail!("Unknown url domain"),
    };

    Ok(connector)
}

pub fn get_task_url_config_key(branch_name: &str) -> String {
    format!("branch.{}.task-url", branch_name)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_returns_right_domain() -> anyhow::Result<()> {
        let connector = parse_repo_connector_url("https://github.com/org/repo.git")?;

        let connector2 = parse_repo_connector_url("https://gitlab.com/some_org/some_repo.git")?;

        let connector3 = parse_repo_connector_url("https://bitbucket.org/fake_org/repo.git")?;

        let connector4 = parse_repo_connector_url("git@github.com:fake_org2/repo2.git")?;

        assert_eq!(connector, RepoConnector::Github("org/repo".to_string()));
        assert_eq!(
            connector2,
            RepoConnector::Gitlab("some_org/some_repo".to_string())
        );
        assert_eq!(
            connector3,
            RepoConnector::Bitbucket("fake_org/repo".to_string())
        );
        assert_eq!(
            connector4,
            RepoConnector::Github("fake_org2/repo2".to_string())
        );

        Ok(())
    }

    #[test]
    fn it_fails_with_unsupported_domain() {
        let result = parse_repo_connector_url("https://some-invalid-domain.com/org/repo.git");

        assert!(result.is_err());

        let error = result.unwrap_err();
        let root_cause = error.root_cause();

        assert_eq!(format!("{}", root_cause), "No regex match found");
    }

    #[test]
    fn it_fails_without_a_project() {
        let result = parse_repo_connector_url("https://github.com");

        assert!(result.is_err());

        let error = result.unwrap_err();
        let root_cause = error.root_cause();

        assert_eq!(format!("{}", root_cause), "No regex match found");
    }
}
