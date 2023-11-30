use crate::{
    connectors::models::TaskDetails, git::GitBranch, strings::stringify_query_params,
    utils::get_current_task_url,
};

pub fn create_github_pull_request_creation_url(
    project_id: &str,
    task_info: &TaskDetails,
) -> anyhow::Result<String> {
    let current_branch = GitBranch::current()?;

    let task_url = get_current_task_url()?;

    Ok(format!(
        "https://github.com/{}/compare/{}?{}",
        project_id,
        current_branch,
        stringify_query_params(vec![
            ("expand", "1"),
            ("title", &task_info.name),
            (
                "body",
                format!(
                    "### Changes\n- [x] {}\n\n---\n\n{}",
                    task_info.name, task_url
                )
                .as_str()
            )
        ])
    ))
}
