use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use std::env;

use crate::{
    constants::ASANA_API_BASE_URL,
    task_connectors::models::{BaseTask, ConnectorType, TaskDetails},
};

#[derive(Debug, Deserialize)]
pub struct AsanaTask {
    pub gid: String,
    pub name: String,
}

impl BaseTask for AsanaTask {
    fn get_details(&self) -> TaskDetails {
        TaskDetails {
            connector: ConnectorType::Asana,
            id: self.gid.to_owned(),
            name: self.name.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub data: T,
}

impl AsanaTask {
    pub async fn fetch(task_id: &str) -> anyhow::Result<Self> {
        let asana_token = env::var("ASANA_TOKEN").context("An ASANA_TOKEN env var is required.")?;

        let response = Client::new()
            .get(format!("{}/tasks/{}", ASANA_API_BASE_URL, task_id))
            .bearer_auth(&asana_token)
            .send()
            .await
            .context(format!(
                "Failed to fetch the asana task with id \"{}\"",
                task_id
            ))?;

        let task: Data<AsanaTask> = response.json().await?;

        Ok(task.data)
    }
}
