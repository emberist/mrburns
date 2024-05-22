use std::env;

use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;

use crate::{
    constants::GITHUB_API_BASE_URL,
    task_connectors::models::{BaseTask, ConnectorType, TaskDetails},
};

#[derive(Debug, Deserialize)]
pub struct GithubIssue {
    title: String,
    number: u64,
}

impl BaseTask for GithubIssue {
    fn get_details(&self) -> TaskDetails {
        TaskDetails {
            connector: ConnectorType::Github,
            id: self.number.to_string(),
            name: self.title.to_owned(),
        }
    }
}

impl GithubIssue {
    pub async fn fetch(repo: &str, issue_id: &u64) -> anyhow::Result<Self> {
        let github_token =
            env::var("GITHUB_TOKEN").context("An GITHUB_TOKEN env var is required.")?;

        let api_url = format!("{}/repos/{}/issues/{}", GITHUB_API_BASE_URL, repo, issue_id);

        let response = Client::new()
            .get(api_url)
            .bearer_auth(&github_token)
            .header("User-Agent", "request")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .context(format!(
                "Failed to fetch the github issue with id \"{}\"",
                issue_id
            ))?;

        let task: GithubIssue = response.json().await?;

        Ok(task)
    }
}
