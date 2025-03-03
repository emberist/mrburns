use regex::Regex;

pub fn make_github_issue_url_regex() -> Regex {
    Regex::new(r"github\.com\/(.+)\/issues\/(\d+)").unwrap()
}

pub fn get_github_issue_info_from_url(url: &str) -> Option<(&str, u64)> {
    match make_github_issue_url_regex().captures(url) {
        Some(caps) => {
            let repo = caps.get(1).map(|m| m.as_str());

            let issue_id = caps.get(2).and_then(|m| m.as_str().parse::<u64>().ok());

            match (repo, issue_id) {
                (Some(repo), Some(issue_id)) => Some((repo, issue_id)),
                _ => None,
            }
        }
        _ => None,
    }
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
            let (repo, task_id) = get_github_issue_info_from_url(fixture.0).unwrap();

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

            assert_eq!(result.is_none(), true);
        }
    }
}
