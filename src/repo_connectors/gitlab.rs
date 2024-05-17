use url::Url;

use crate::{
    config::Config, git::GitBranch, task_connectors::models::TaskDetails,
    utils::get_current_task_url,
};

pub struct GitlabRepo {}

impl GitlabRepo {
    pub fn create_mr_url(
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

        let gitlab_merge_request_url =
            format!("https://gitlab.com/{}/merge_requests/new", project_id);

        let url = Url::parse_with_params(
            &gitlab_merge_request_url,
            &[
                (
                    "merge_request[title]",
                    format!("Draft: {}", task_info.name).as_str(),
                ),
                ("merge_request[source_branch]", &current_branch_name),
                ("merge_request[target_branch]", &target_branch),
                ("merge_request[description]", &description),
                (
                    "merge_request[draft]",
                    format!("{}", config.create_draft_mr).as_str(),
                ),
                ("merge_request[squash_on_merge]", "true"),
                ("merge_request[remove_source_branch]", "true"),
            ],
        )?;

        Ok(url.to_string())
    }
}
