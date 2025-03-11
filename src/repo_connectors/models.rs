use std::path::Path;

use anyhow::bail;
use anyhow::Context;
use regex::Regex;

use crate::config::get_default_mr_description;
use crate::config::MrburnsConfig;
use crate::constants::DEFAULT_MR_TEMPLATE_PATH;
use crate::constants::TASK_ACTIONS_REF;
use crate::constants::TASK_ID_REF;
use crate::git::client::GitClient;
use crate::task_connectors::models::TaskDetails;

use super::github::GithubRepo;
use super::gitlab::GitlabRepo;

#[derive(Debug, PartialEq)]
pub enum RepoConnector {
    Github(String),
    Gitlab(String),
    Bitbucket(String),
}

impl RepoConnector {
    pub fn from_remote(url: &str) -> anyhow::Result<RepoConnector> {
        let captures = Regex::new(
            r"^(?:git@|https:\/\/)(gitlab\.com|github\.com|bitbucket\.org)(?::|\/)(.+)\.git$",
        )?
        .captures(url)
        .ok_or_else(|| anyhow::anyhow!("No regex match found"))?;

        let url_domain = captures.get(1).context("No match found")?.as_str();

        let project = captures
            .get(2)
            .context("No project id found")?
            .as_str()
            .to_string();

        let connector = match url_domain {
            "github.com" => RepoConnector::Github(project),
            "gitlab.com" => RepoConnector::Gitlab(project),
            "bitbucket.org" => RepoConnector::Bitbucket(project),
            _ => bail!("Unknown url domain"),
        };

        Ok(connector)
    }

    pub fn create_mr_url(&self, task: &TaskDetails, target_branch: &str) -> anyhow::Result<String> {
        let git_client = GitClient {};

        let config = MrburnsConfig::read();

        let actions = match self {
            RepoConnector::Github(_) => format!("closes #{}", TASK_ID_REF),
            RepoConnector::Gitlab(_) => "/assign me".to_string(),
            RepoConnector::Bitbucket(_) => String::default(),
        };

        let default_description_template =
            get_default_mr_description().replace(TASK_ACTIONS_REF, &actions);

        let description_template = if Path::new(DEFAULT_MR_TEMPLATE_PATH).exists() {
            std::fs::read_to_string(DEFAULT_MR_TEMPLATE_PATH)
                .unwrap_or(default_description_template)
                .replace(TASK_ACTIONS_REF, &actions)
        } else {
            default_description_template
        };

        let url = match self {
            RepoConnector::Github(project) => GithubRepo::create_mr_url(
                &config,
                &git_client,
                project,
                task,
                target_branch,
                &description_template,
            )?,
            RepoConnector::Gitlab(project) => GitlabRepo::create_mr_url(
                &config,
                &git_client,
                project,
                task,
                target_branch,
                &description_template,
            )?,
            _ => bail!("Not implemented yet."),
        };

        Ok(url)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_returns_right_domain() -> anyhow::Result<()> {
        let connector = RepoConnector::from_remote("https://github.com/org/repo.git")?;

        let connector2 = RepoConnector::from_remote("https://gitlab.com/some_org/some_repo.git")?;

        let connector3 = RepoConnector::from_remote("https://bitbucket.org/fake_org/repo.git")?;

        let connector4 = RepoConnector::from_remote("git@github.com:fake_org2/repo2.git")?;

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
        let result = RepoConnector::from_remote("https://some-invalid-domain.com/org/repo.git");

        assert!(result.is_err());

        let error = result.unwrap_err();
        let root_cause = error.root_cause();

        assert_eq!(format!("{}", root_cause), "No regex match found");
    }

    #[test]
    fn it_fails_without_a_project() {
        let result = RepoConnector::from_remote("https://github.com");

        assert!(result.is_err());

        let error = result.unwrap_err();
        let root_cause = error.root_cause();

        assert_eq!(format!("{}", root_cause), "No regex match found");
    }
}
