use std::env;

use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;

use crate::{
    constants::CLICKUP_API_BASE_URL,
    task_connectors::models::{BaseTask, ConnectorType, TaskDetails},
};

#[derive(Debug, Deserialize)]
pub struct ClickupTask {
    id: String,
    name: String,
}

impl BaseTask for ClickupTask {
    fn get_details(&self) -> TaskDetails {
        TaskDetails {
            connector: ConnectorType::ClickUp,
            id: self.id.to_owned(),
            name: self.name.to_owned(),
        }
    }
}

impl ClickupTask {
    pub async fn fetch(task_id: &String) -> anyhow::Result<Self> {
        let clickup_token =
            env::var("CLICKUP_TOKEN").context("An CLICKUP_TOKEN env var is required.")?;

        let api_url = format!("{}/api/v2/task/{}", CLICKUP_API_BASE_URL, task_id);

        let response = Client::new()
            .get(api_url)
            .header("Authorization", &clickup_token)
            .send()
            .await
            .context(format!(
                "Failed to fetch the clickup task with id \"{}\"",
                task_id
            ))?;

        let task: ClickupTask = response.json().await?;

        Ok(task)
    }
}
