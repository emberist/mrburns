use regex::Regex;

use crate::{
    connectors::models::TaskDetails, git::GitBranch, strings::stringify_query_params,
    utils::get_current_task_url,
};

pub fn get_github_issue_url_regex() -> Regex {
    Regex::new(r"github\.com\/(.+)\/issues\/(\d+)").unwrap()
}

pub fn create_github_pull_request_creation_url(
    project_id: &str,
    task_info: &TaskDetails,
    target_branch: &str,
) -> anyhow::Result<String> {
    let current_branch = GitBranch::current()?;

    let task_url = get_current_task_url()?;

    Ok(format!(
        "https://github.com/{}/compare/{}...{}?{}",
        project_id,
        target_branch,
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

pub fn get_github_issue_info_from_url(url: &str) -> anyhow::Result<(&str, u64)> {
    let captures = get_github_issue_url_regex()
        .captures(url)
        .ok_or_else(|| anyhow::anyhow!("No regex match found"))?;

    let repo = captures
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Match not exists"))?
        .as_str();

    let issue_id = captures
        .get(2)
        .ok_or_else(|| anyhow::anyhow!("Match not exists"))?
        .as_str()
        .parse::<u64>()?;

    Ok((repo, issue_id))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_returns_the_right_task_id() -> anyhow::Result<()> {
        let fixtures = [
            (
                "https://github.com/owner/repo/issues/12",
                ("owner/repo", 12),
            ),
            (
                "https://github.com/owner/repo2/issues/13",
                ("owner/repo2", 13),
            ),
            ("https://github.com/something/issues/13", ("something", 13)),
        ];

        for fixture in fixtures.iter() {
            let (repo, task_id) = get_github_issue_info_from_url(fixture.0)?;

            assert_eq!(repo, fixture.1 .0);
            assert_eq!(task_id, fixture.1 .1);
        }

        Ok(())
    }

    #[test]
    fn it_fails_getting_task_id() {
        let fixtures = [
            "https://github.com/owner/repo/12",
            "https://github.com/owner/repo2/issues/ciao",
            "https://github.com/issues/13",
        ];

        for fixture in fixtures.iter() {
            let result = get_github_issue_info_from_url(fixture);

            let error = result.unwrap_err();

            assert_eq!(error.to_string(), "No regex match found".to_string());
        }
    }
}
