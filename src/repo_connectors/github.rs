use url::Url;

use crate::{
    config::MrburnsConfig,
    constants::{TASK_DESCRIPTION_REF, TASK_ID_REF, TASK_TITLE_REF, TASK_TYPE_REF, TASK_URL_REF},
    git::adapter::GitClientAdapter,
    task_connectors::models::TaskDetails,
    utils::get_current_task_url,
};

pub struct GithubRepo {}

impl GithubRepo {
    pub fn create_mr_url(
        config: &MrburnsConfig,
        git_client: &impl GitClientAdapter,
        project_id: &str,
        task_info: &TaskDetails,
        target_branch: &str,
        description_template: &str,
    ) -> anyhow::Result<String> {
        let current_branch = git_client.current_branch()?;

        let task_url = get_current_task_url(git_client)?;

        let github_base_url = format!(
            "https://github.com/{}/compare/{}...{}",
            project_id, target_branch, current_branch
        );

        let task_type = current_branch
            .split("/")
            .next()
            .unwrap_or(&config.branches.prefixes.feature);

        let mr_title = config
            .mr
            .title_format
            .replace(TASK_ID_REF, &task_info.id)
            .replace(TASK_TYPE_REF, &task_type)
            .replace(TASK_TITLE_REF, &task_info.name);

        let mr_description = description_template
            .replace(TASK_ID_REF, &task_info.id)
            .replace(TASK_TYPE_REF, &task_type)
            .replace(TASK_TITLE_REF, &task_info.name)
            .replace(
                TASK_DESCRIPTION_REF,
                &task_info.description.clone().unwrap_or_default(),
            )
            .replace(TASK_URL_REF, &task_url);

        let url = Url::parse_with_params(
            &github_base_url,
            [
                ("expand", "1"),
                ("title", &mr_title),
                ("body", mr_description.as_str()),
            ],
        )?;

        Ok(url.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use anyhow::Result;

    use crate::{
        config::get_default_mr_description, git::mock::GitClientMock,
        task_connectors::models::ConnectorType,
    };

    use super::*;

    #[test]
    fn creates_github_mr_url() -> Result<()> {
        let config = MrburnsConfig::default();
        let mr_url = GithubRepo::create_mr_url(
            &config,
            &GitClientMock {},
            "owner/repo",
            &TaskDetails {
                connector: ConnectorType::Github,
                id: "123".to_string(),
                name: "Hello World".to_string(),
                description: Some("Some cool description".to_string()),
            },
            "master",
            &get_default_mr_description(),
        )
        .unwrap();

        let url = Url::parse(&mr_url)?;

        assert_eq!(url.domain(), Some("github.com"));
        assert_eq!(
            url.path(),
            "/owner/repo/compare/master...feat/some-cool-feature"
        );

        let params = url.query_pairs().collect::<Vec<_>>();

        let expected_params = [
            ("expand", "1"),
            ("title", "123/feat/Hello World"),
            ("body", "# [Hello World](https://github.com/some-cool-repo/issues/0)\n## Description\nSome cool description\n\n---\n{task_actions}")
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
