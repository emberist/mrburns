use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use std::env;

use crate::config::Config;

#[derive(Deserialize, Debug)]
pub struct Fields {
    pub summary: String,
}

#[derive(Deserialize, Debug)]
pub struct JiraIssue {
    pub fields: Fields,
    pub key: String,
}

pub async fn fetch_jira_task_by_issue_id(issue_id: &str) -> anyhow::Result<JiraIssue> {
    let jira_user = env::var("JIRA_USERNAME").expect("A JIRA_USERNAME env var is required");
    let jira_token = env::var("JIRA_TOKEN").expect("A JIRA_TOKEN env var is required");

    let jira_api_base_url = Config::read()?
        .jira_api_base_url
        .context("No Jira url found in config. Use `mrburns config` to add a configuration...")?;

    let jira_api_url = format!("{}/rest/api/latest/issue/{}", jira_api_base_url, issue_id);

    let response = Client::new()
        .get(&jira_api_url)
        .basic_auth(jira_user.clone(), Some(jira_token.clone()))
        .send()
        .await?;

    let issue: JiraIssue = response.json().await?;

    Ok(issue)
}
