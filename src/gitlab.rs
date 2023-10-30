use reqwest::Client;
use serde::Deserialize;
use std::env;

use crate::connectors::task::fetch_connector_task;

#[derive(Deserialize, Debug)]
pub struct Author {
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct CreateMrResponse {
    pub iid: i32,
    pub web_url: String,
    pub author: Author,
}

pub async fn create_gitlab_mr(
    project_id: &str,
    task_url: &str,
    source_branch: &str,
    target_branch: &str,
    draft: bool,
) -> anyhow::Result<CreateMrResponse> {
    let gitlab_access_token = env::var("GITLAB_TOKEN").expect("A GITLAB_TOKEN env var is required");

    let url_encoded_project_id = urlencoding::encode(project_id).to_string();

    let gitlab_api_url = format!(
        "https://gitlab.com/api/v4/projects/{}/merge_requests",
        url_encoded_project_id
    );

    let jira_issue = fetch_connector_task(task_url).await?;

    let description = format!(
        r"### Changes\n- [x] {}\n\n---\n\n{}",
        jira_issue.name, task_url
    );

    let body = format!(
        r#"{{ "title": "Draft: {}", "description": "{}", "source_branch": "{}", "target_branch": "{}", "draft": {}, "remove_source_branch": true, "squash_on_merge": true }}"#,
        source_branch, description, source_branch, target_branch, draft
    );

    let response = Client::new()
        .post(&gitlab_api_url)
        .header(
            "Authorization",
            "Bearer ".to_owned() + gitlab_access_token.as_str(),
        )
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let mr_response: CreateMrResponse = response.json().await?;

    Ok(mr_response)
}

pub async fn add_merge_request_author(
    project_id: &str,
    merge_request_iid: i32,
    author_id: i32,
) -> anyhow::Result<()> {
    let gitlab_access_token = env::var("GITLAB_TOKEN").expect("A GITLAB_TOKEN env var is required");

    let url_encoded_project_id = urlencoding::encode(project_id).to_string();

    let gitlab_api_url = format!(
        "https://gitlab.com/api/v4/projects/{}/merge_requests/{}",
        url_encoded_project_id.as_str(),
        merge_request_iid
    );

    let body = format!(r#"{{ "assignee_id": {} }}"#, author_id);

    let response = Client::new()
        .put(&gitlab_api_url)
        .header(
            "Authorization",
            "Bearer ".to_owned() + gitlab_access_token.as_str(),
        )
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    response.text().await?;

    Ok(())
}
