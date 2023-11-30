use anyhow::Context;
use reqwest::Client;
use std::env;

use crate::constants::GITHUB_API_BASE_URL;

use super::models::GithubIssue;

fn get_github_api_url(repo: &str, issue_id: &u64) -> String {
    format!("{}/repos/{}/issues/{}", GITHUB_API_BASE_URL, repo, issue_id)
}

pub async fn fetch_github_issue_by_id(repo: &str, issue_id: &u64) -> anyhow::Result<GithubIssue> {
    let github_token = env::var("GITHUB_TOKEN").context("An GITHUB_TOKEN env var is required.")?;

    let api_url = get_github_api_url(repo, issue_id);

    let response = Client::new()
        .get(api_url)
        .bearer_auth(&github_token)
        .header("User-Agent", "request")
        .header("Accept", "application/vnd.github+json")
        // .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .context(format!(
            "Failed to fetch the github issue with id \"{}\"",
            issue_id
        ))?;

    let task: GithubIssue = response.json().await?;

    Ok(task)
}
