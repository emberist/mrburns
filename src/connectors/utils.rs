use crate::{
    asana::utils::{get_asana_task_id_from_url, get_asana_task_url_regex},
    jira::utils::{
        get_jira_task_domain_from_url, get_jira_task_id_from_url, get_jira_task_url_regex,
    },
};

use super::models::TaskConnector;

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
