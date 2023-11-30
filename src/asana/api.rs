use anyhow::Context;
use reqwest::Client;
use std::env;

use crate::{asana::models::Data, constants::ASANA_API_BASE_URL};

use super::models::AsanaTask;

pub async fn fetch_asana_task_by_id(task_id: &str) -> anyhow::Result<AsanaTask> {
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
