use anyhow::bail;

use crate::{
    github::utils::create_github_pull_request_creation_url,
    gitlab::create_gitlab_merge_request_creation_url, strings::slugify,
};

#[derive(Debug, Clone)]

pub enum TaskConnector {
    Asana(String),
    Jira {
        api_base_url: String,
        task_id: String,
    },
    Github {
        repo: String,
        issue_id: u64,
    },
}

pub struct TaskDetails {
    pub connector: TaskConnector,
    pub name: String,
}

impl TaskDetails {
    pub fn sanitized_name(&self) -> String {
        let connector = self.connector.to_owned();
        let normalized_task_name = slugify(&self.name);

        match connector {
            TaskConnector::Asana(_) => normalized_task_name,
            TaskConnector::Github { .. } => normalized_task_name,
            TaskConnector::Jira { task_id, .. } => {
                format!("{}-{}", task_id, normalized_task_name)
            }
        }
    }
}

pub trait Task {
    fn info(&self, connector: TaskConnector) -> TaskDetails;
}

#[derive(Debug, PartialEq)]
pub enum RepoConnector {
    Github(String),
    Gitlab(String),
    Bitbucket(String),
}

pub trait Mergeble {
    fn mr_url(&self, task: &TaskDetails, target_branch: &str) -> anyhow::Result<String>;
}

impl Mergeble for RepoConnector {
    fn mr_url(&self, task: &TaskDetails, target_branch: &str) -> anyhow::Result<String> {
        let url = match self {
            RepoConnector::Github(project) => {
                create_github_pull_request_creation_url(project, task, target_branch)?
            }
            RepoConnector::Gitlab(project) => {
                create_gitlab_merge_request_creation_url(project, task, target_branch)?
            }
            _ => bail!("Not implemented yet."),
        };

        Ok(url)
    }
}
