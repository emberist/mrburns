use url::Url;

use crate::{
    config::Config,
    constants::{TASK_ID_REF, TASK_TITLE_REF, TASK_TYPE_REF, TASK_URL_REF},
    git::adapter::GitClientAdapter,
    task_connectors::models::TaskDetails,
    utils::get_current_task_url,
};

pub struct GitlabRepo {}

impl GitlabRepo {
    pub fn create_mr_url(
        config: &Config,
        git_client: &impl GitClientAdapter,
        project_id: &str,
        task_info: &TaskDetails,
        target_branch: &str,
        description_template: &str,
    ) -> anyhow::Result<String> {
        let current_branch_name = git_client.current_branch()?;

        let task_url = get_current_task_url(git_client)?;

        let gitlab_merge_request_url =
            format!("https://gitlab.com/{}/merge_requests/new", project_id);

        let current_branch = git_client.current_branch()?;

        let task_type = current_branch
            .split("/")
            .next()
            .unwrap_or(&config.branch_prefixes.feature);

        let mr_title = config
            .mr
            .title_template
            .replace(TASK_ID_REF, &task_info.id)
            .replace(TASK_TYPE_REF, &task_type)
            .replace(TASK_TITLE_REF, &task_info.name);

        let mr_description = description_template
            .replace(TASK_ID_REF, &task_info.id)
            .replace(TASK_TYPE_REF, &task_type)
            .replace(TASK_TITLE_REF, &task_info.name)
            .replace(TASK_URL_REF, &task_url);

        let url = Url::parse_with_params(
            &gitlab_merge_request_url,
            &[
                ("merge_request[title]", mr_title.as_str()),
                ("merge_request[source_branch]", &current_branch_name),
                ("merge_request[target_branch]", &target_branch),
                ("merge_request[description]", &mr_description),
                (
                    "merge_request[draft]",
                    config.mr.default_draft.to_string().as_str(),
                ),
                ("merge_request[squash_on_merge]", "true"),
                ("merge_request[remove_source_branch]", "true"),
            ],
        )?;

        Ok(url.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use anyhow::Result;

    use crate::{git::mock::GitClientMock, task_connectors::models::ConnectorType};

    use super::*;

    #[test]
    fn creates_gitlab_mr_url() -> Result<()> {
        let config = Config::default();

        let mr_url = GitlabRepo::create_mr_url(
            &config,
            &GitClientMock {},
            "owner/repo",
            &TaskDetails {
                connector: ConnectorType::Jira,
                id: "123".to_string(),
                name: "Hello World".to_string(),
            },
            "master",
            &config.mr.description_template.join("\n"),
        )
        .unwrap();

        let url = Url::parse(&mr_url)?;

        assert_eq!(url.domain(), Some("gitlab.com"));
        assert_eq!(url.path(), "/owner/repo/merge_requests/new");

        let params = url.query_pairs().collect::<Vec<_>>();

        let expected_params = [
            ("merge_request[title]", "123/feat/Hello World"),
            ("merge_request[source_branch]", "feat/some-cool-feature"),
            ("merge_request[target_branch]", "master"),
            (
                "merge_request[description]",
                "### Changes\n- [x] [Hello World](https://github.com/some-cool-repo/issues/0)\n\n---\n{task_actions}",
            ),
            ("merge_request[draft]", "false"),
            ("merge_request[squash_on_merge]", "true"),
            ("merge_request[remove_source_branch]", "true"),
        ];

        zip(params, expected_params).for_each(
            |((title, value), (expected_title, expected_value))| {
                assert_eq!(title, expected_title);
                assert_eq!(value, expected_value);
            },
        );

        Ok(())
    }
}
