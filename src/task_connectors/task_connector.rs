use crate::{
    git::adapter::GitClientAdapter,
    task_connectors::{
        asana::utils::get_asana_task_id_from_url, github::utils::get_github_issue_info_from_url,
        jira::utils::get_jira_task_info_from_url,
    },
    utils::get_current_task_url,
};

use super::{
    asana::models::AsanaTask,
    github::models::GithubIssue,
    jira::models::JiraTask,
    models::{BaseTask, TaskDetails},
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TaskConnector {
    Asana {
        original_url: String,
        task_id: String,
    },
    Jira {
        original_url: String,
        api_base_url: String,
        task_id: String,
    },
    Github {
        original_url: String,
        repo: String,
        issue_id: u64,
    },
}

impl TaskConnector {
    pub fn from_task_url(git: &impl GitClientAdapter) -> anyhow::Result<Self> {
        let task_url = get_current_task_url(git)?;

        Self::from_url(&task_url)
    }

    pub fn from_url(url: &str) -> anyhow::Result<Self> {
        if let Some(matched_task) = get_asana_task_id_from_url(url) {
            return Ok(TaskConnector::Asana {
                original_url: url.to_owned(),
                task_id: matched_task.to_string(),
            });
        }

        if let Some((api_base_url, task_id)) = get_jira_task_info_from_url(url) {
            return Ok(TaskConnector::Jira {
                original_url: url.to_owned(),
                api_base_url: api_base_url.to_string(),
                task_id: task_id.to_string(),
            });
        }

        if let Some((repo, issue_id)) = get_github_issue_info_from_url(url) {
            return Ok(TaskConnector::Github {
                original_url: url.to_owned(),
                repo: repo.to_string(),
                issue_id,
            });
        }

        anyhow::bail!("Url does not match any task connectors")
    }

    pub async fn fetch_task(self) -> anyhow::Result<TaskDetails> {
        match self {
            Self::Asana { task_id, .. } => {
                let task = AsanaTask::fetch(&task_id).await?;

                Ok(task.get_details())
            }
            Self::Jira {
                api_base_url,
                task_id,
                ..
            } => {
                let task = JiraTask::fetch(&api_base_url, &task_id).await?;

                Ok(task.get_details())
            }
            Self::Github { repo, issue_id, .. } => {
                let task = GithubIssue::fetch(&repo, &issue_id).await?;

                Ok(task.get_details())
            }
        }
    }
}
