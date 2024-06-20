use url::Url;

use crate::{git::Git, task_connectors::models::TaskDetails, utils::get_current_task_url};

pub struct GithubRepo {}

impl GithubRepo {
    pub fn create_mr_url(
        project_id: &str,
        task_info: &TaskDetails,
        target_branch: &str,
    ) -> anyhow::Result<String> {
        let current_branch = Git::current_branch()?;

        let task_url = get_current_task_url()?;

        let github_base_url = format!(
            "https://github.com/{}/compare/{}...{}",
            project_id, target_branch, current_branch
        );

        let url = Url::parse_with_params(
            &github_base_url,
            [
                ("expand", "1"),
                ("title", &task_info.name),
                (
                    "body",
                    format!(
                        "### Changes\n- [x] {}\n\n---\n\n{}",
                        task_info.name, task_url
                    )
                    .as_str(),
                ),
            ],
        )?;

        Ok(url.to_string())
    }
}
