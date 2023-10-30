use anyhow::Context;
use reqwest::Client;
use std::env;

use crate::config::Config;

use super::models::JiraTask;

pub fn get_jira_api_url(task_id: &str) -> anyhow::Result<String> {
    let jira_api_base_url = Config::read()?
        .jira_api_base_url
        .ok_or_else(|| anyhow::anyhow!("Jira api base url not found"))?;

    Ok(format!(
        "{}/rest/api/latest/issue/{}",
        jira_api_base_url, task_id
    ))
}

pub async fn fetch_jira_task_by_id(task_id: &str) -> anyhow::Result<JiraTask> {
    let jira_user = env::var("JIRA_USERNAME").context("An JIRA_USERNAME env var is required.")?;
    let jira_token = env::var("JIRA_TOKEN").context("An JIRA_TOKEN env var is required.")?;

    let jira_api_base_url = get_jira_api_url(task_id)?;

    let jira_api_url = format!("{}/rest/api/latest/issue/{}", jira_api_base_url, task_id);

    let response = Client::new()
        .get(&jira_api_url)
        .basic_auth(jira_user.clone(), Some(jira_token.clone()))
        .send()
        .await
        .context(format!(
            "Failed to fetch the jira task with id \"{}\"",
            task_id
        ))?;

    let issue = response.json().await?;

    Ok(issue)
}