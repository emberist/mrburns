use crate::{asana::api::fetch_asana_task_by_id, jira::api::fetch_jira_task_by_id};

use super::{
    models::{TaskConnector, TaskConnectorTrait, TaskInfo},
    utils::parse_task_connector_url,
};

pub async fn fetch_connector_task(task_url: &str) -> anyhow::Result<TaskInfo> {
    let parsed_url = parse_task_connector_url(task_url)?;

    match &parsed_url {
        TaskConnector::Asana(task_id) => {
            let asana_task = fetch_asana_task_by_id(task_id).await?;

            Ok(asana_task.get_info())
        }
        TaskConnector::Jira(task_id) => {
            let jira_task = fetch_jira_task_by_id(task_id).await?;

            Ok(jira_task.get_info())
        }
    }
}
