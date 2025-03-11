use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use std::env;

use crate::task_connectors::models::{BaseTask, ConnectorType, TaskDetails};

#[derive(Deserialize, Debug)]
pub struct Fields {
    pub summary: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct JiraTask {
    pub fields: Fields,
    pub key: String,
}

impl BaseTask for JiraTask {
    fn get_details(&self) -> TaskDetails {
        TaskDetails {
            connector: ConnectorType::Jira,
            description: self.fields.description.to_owned(),
            id: self.key.to_owned(),
            name: self.fields.summary.to_owned(),
        }
    }
}

impl JiraTask {
    pub async fn fetch(api_base_url: &str, task_id: &str) -> anyhow::Result<Self> {
        let jira_user =
            env::var("JIRA_USERNAME").context("An JIRA_USERNAME env var is required.")?;

        let jira_token = env::var("JIRA_TOKEN").context("An JIRA_TOKEN env var is required.")?;

        let jira_api_url = format!("{}/rest/api/latest/issue/{}", api_base_url, task_id);

        let response = Client::new()
            .get(&jira_api_url)
            .basic_auth(jira_user.clone(), Some(jira_token.clone()))
            .send()
            .await
            .context(format!(
                "Failed to fetch the jira task with id \"{}\"",
                task_id
            ))?;

        let issue = response.json().await.context("Failed to decode JiraTask")?;

        Ok(issue)
    }
}
