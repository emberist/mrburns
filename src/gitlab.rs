use crate::{
    config::Config, connectors::models::TaskDetails, git::GitBranch,
    strings::stringify_query_params, utils::get_current_task_url,
};

pub fn create_gitlab_merge_request_creation_url(
    project_id: &str,
    task_info: &TaskDetails,
    target_branch: &str,
) -> anyhow::Result<String> {
    let current_branch_name = GitBranch::current()?;

    let task_url = get_current_task_url()?;

    let config = Config::read()?;

    let description = format!(
        "### Changes\n- [x] {}\n\n---\n\n{}",
        task_info.name, task_url
    );

    Ok(format!(
        "https://gitlab.com/{}/merge_requests/new?{}",
        project_id,
        stringify_query_params(vec![
            (
                "merge_request[title]",
                format!("Draft: {}", task_info.name).as_str()
            ),
            ("merge_request[source_branch]", &current_branch_name),
            ("merge_request[target_branch]", &target_branch),
            ("merge_request[description]", &description),
            (
                "merge_request[draft]",
                format!("{}", config.create_draft_mr).as_str()
            ),
            ("merge_request[squash_on_merge]", "true"),
            ("merge_request[remove_source_branch]", "true"),
        ])
    ))
}
