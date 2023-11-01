use anyhow::Context;
use regex::Regex;

use crate::{
    asana::utils::{get_asana_task_id_from_url, get_asana_task_url_regex},
    jira::utils::{
        get_jira_task_domain_from_url, get_jira_task_id_from_url, get_jira_task_url_regex,
    },
};

use super::models::{RepoInfo, RepoProvider, TaskConnector};

pub fn parse_task_connector_url(url: &str) -> anyhow::Result<TaskConnector> {
    if get_asana_task_url_regex().is_match(url) {
        let task_id = get_asana_task_id_from_url(url)?;

        return Ok(TaskConnector::Asana(task_id.to_string()));
    }

    if get_jira_task_url_regex().is_match(url) {
        let task_id = get_jira_task_id_from_url(url)?;
        let api_base_url = get_jira_task_domain_from_url(url)?;

        return Ok(TaskConnector::Jira {
            api_base_url: api_base_url.to_string(),
            task_id: task_id.to_string(),
        });
    }

    anyhow::bail!("Url does not match any connectors")
}

pub fn parse_repo_url(url: &str) -> anyhow::Result<RepoInfo> {
    let captures = Regex::new(
        r"^(?:git@|https:\/\/)(gitlab\.com|github\.com|bitbucket\.org)(?::|\/)(.+)\.git$",
    )?
    .captures(url)
    .ok_or_else(|| anyhow::anyhow!("No regex match found"))?;

    let url_domain = captures.get(1).expect("No  match found").as_str();

    let domain = match url_domain {
        "github.com" => RepoProvider::Github,
        "gitlab.com" => RepoProvider::Gitlab,
        "bitbucket.org" => RepoProvider::Bitbucket,
        _ => anyhow::bail!("Unknown url domain"),
    };

    let project = captures
        .get(2)
        .context("No project id found")?
        .as_str()
        .to_string();

    Ok(RepoInfo {
        provider: domain,
        project,
    })
}

pub fn get_task_url_config_key(branch_name: &str) -> String {
    format!("branch.{}.task-url", branch_name)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_returns_right_domain() -> anyhow::Result<()> {
        let RepoInfo {
            provider: domain,
            project,
        } = parse_repo_url("https://github.com/org/repo.git")?;

        let RepoInfo {
            provider: domain2,
            project: project2,
        } = parse_repo_url("https://gitlab.com/some_org/some_repo.git")?;

        let RepoInfo {
            provider: domain3,
            project: project3,
        } = parse_repo_url("https://bitbucket.org/fake_org/repo.git")?;

        let RepoInfo {
            provider: domain4,
            project: project4,
        } = parse_repo_url("git@github.com:fake_org2/repo2.git")?;

        assert_eq!(domain, RepoProvider::Github);
        assert_eq!(domain2, RepoProvider::Gitlab);
        assert_eq!(domain3, RepoProvider::Bitbucket);
        assert_eq!(domain4, RepoProvider::Github);

        assert_eq!(project, "org/repo");
        assert_eq!(project2, "some_org/some_repo");
        assert_eq!(project3, "fake_org/repo");
        assert_eq!(project4, "fake_org2/repo2");

        Ok(())
    }

    #[test]
    fn it_fails_with_unsupported_domain() {
        let result = parse_repo_url("https://some-invalid-domain.com/org/repo.git");

        assert!(result.is_err());

        let error = result.unwrap_err();
        let root_cause = error.root_cause();

        assert_eq!(format!("{}", root_cause), "No regex match found");
    }

    #[test]
    fn it_fails_without_a_project() {
        let result = parse_repo_url("https://github.com");

        assert!(result.is_err());

        let error = result.unwrap_err();
        let root_cause = error.root_cause();

        assert_eq!(format!("{}", root_cause), "No regex match found");
    }
}
