use regex::Regex;

pub fn make_jira_task_url_regex() -> Regex {
    Regex::new(r"^((?:https?:\/\/)?[^\/]+\.atlassian\.net)\/browse\/([^\/]+)$").unwrap()
}

pub fn get_jira_task_info_from_url(url: &str) -> Option<(&str, &str)> {
    match make_jira_task_url_regex().captures(url) {
        Some(caps) => {
            let task_domain = caps.get(1).map(|m| m.as_str());
            let task_id = caps.get(2).map(|m| m.as_str());

            match (task_domain, task_id) {
                (Some(domain), Some(id)) => Some((domain, id)),
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
                "https://some-domain.atlassian.net/browse/some-task-id",
                ("https://some-domain.atlassian.net", "some-task-id"),
            ),
            (
                "https://some-domain.atlassian.net/browse/some-other-task-id",
                ("https://some-domain.atlassian.net", "some-other-task-id"),
            ),
            (
                "https://a.atlassian.net/browse/1234",
                ("https://a.atlassian.net", "1234"),
            ),
        ];

        for fixture in fixtures.iter() {
            let (domain, task_id) = get_jira_task_info_from_url(fixture.0).unwrap();

            assert_eq!(domain, fixture.1 .0);
            assert_eq!(task_id, fixture.1 .1);
        }

        Ok(())
    }

    #[test]
    fn it_fails_getting_task_id() {
        let fixtures = [
            "https://app.foobar.com/0/0/1200371619465230/f",
            "https::///app.foobar.com/0/0/1200371619465230/f",
            "https:://app.foobar.com/0/0/1200371619465230/f",
            ".atlassian.net/browse/some-task-id",
            "some-domain.atlassian.net/browse/some-task-id/other-info",
            "some-domain.atlassian.net/browse",
            "some-domain.atlassian.net/browse/",
            "foobar",
            "1200371619465230",
        ];

        for fixture in fixtures.iter() {
            let result = get_jira_task_info_from_url(fixture);

            assert_eq!(result.is_none(), true);
        }
    }
}
