use anyhow::Context;
use reqwest::Client;
use std::env;

use crate::{
    connectors::{models::TaskInfo, task::fetch_connector_task},
    constants::GITLAB_API_BASE_URL,
};

use super::models::{CreateMrBody, CreateMrResponse};

fn get_gitlab_merge_requests_api_url(project_id: &str) -> String {
    let url_encoded_project_id = urlencoding::encode(project_id).to_string();

    format!(
        "{}/projects/{}/merge_requests",
        GITLAB_API_BASE_URL, url_encoded_project_id
    )
}

pub async fn create_gitlab_mr(
    project_id: &str,
    task_info: &TaskInfo,
    source_branch: &str,
    target_branch: &str,
    draft: bool,
) -> anyhow::Result<CreateMrResponse> {
    let gitlab_access_token =
        env::var("GITLAB_TOKEN").context("A GITLAB_TOKEN env var is required")?;

    let gitlab_api_url = get_gitlab_merge_requests_api_url(project_id);

    let jira_issue = fetch_connector_task(&task_info.url).await?;

    let description = format!(
        r"### Changes\n- [x] {}\n\n---\n\n{}",
        jira_issue.name, task_info.url
    );

    let body = CreateMrBody {
        title: format!("Draft: {}", source_branch),
        description,
        source_branch: source_branch.to_owned(),
        target_branch: target_branch.to_owned(),
        draft,
        remove_source_branch: true,
        squash_on_merge: true,
    };

    let body_str = serde_json::to_string(&body)?;

    let response = Client::new()
        .post(&gitlab_api_url)
        .bearer_auth(&gitlab_access_token)
        .header("Content-Type", "application/json")
        .body(body_str)
        .send()
        .await?;

    let mr_response: CreateMrResponse = response.json().await?;

    Ok(mr_response)
}

pub async fn add_gitlab_merge_request_author(
    project_id: &str,
    merge_request_iid: i32,
    author_id: i32,
) -> anyhow::Result<()> {
    let gitlab_access_token =
        env::var("GITLAB_TOKEN").context("A GITLAB_TOKEN env var is required")?;

    let gitlab_api_url = format!(
        "{}/{}",
        get_gitlab_merge_requests_api_url(project_id),
        merge_request_iid
    );

    let body = format!(r#"{{ "assignee_id": {} }}"#, author_id);

    let response = Client::new()
        .put(&gitlab_api_url)
        .bearer_auth(&gitlab_access_token)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    response.text().await?;

    Ok(())
}
