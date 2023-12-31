use crate::{
    asana::api::fetch_asana_task_by_id, github::api::fetch_github_issue_by_id,
    jira::api::fetch_jira_task_by_id,
};

use super::{
    models::{Task, TaskConnector, TaskDetails},
    utils::parse_task_connector_url,
};

pub async fn fetch_connector_task(task_url: &str) -> anyhow::Result<TaskDetails> {
    let connector = parse_task_connector_url(task_url)?;

    match &connector {
        TaskConnector::Asana(task_id) => {
            let asana_task = fetch_asana_task_by_id(task_id).await?;

            Ok(asana_task.info(connector))
        }
        TaskConnector::Jira {
            api_base_url,
            task_id,
        } => {
            let jira_task = fetch_jira_task_by_id(api_base_url, task_id).await?;

            Ok(jira_task.info(connector))
        }
        TaskConnector::Github { issue_id, repo } => {
            let github_issue = fetch_github_issue_by_id(repo, issue_id).await?;

            Ok(github_issue.info(connector))
        }
    }
}
