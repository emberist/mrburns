use regex::Regex;

pub fn get_jira_task_url_regex() -> Regex {
    Regex::new(r"^(?:https?:\/\/)?[^\/]+\.atlassian\.net\/browse\/([^\/]+)$").unwrap()
}

pub fn get_jira_task_id_from_url(url: &str) -> anyhow::Result<&str> {
    let matched_task = get_jira_task_url_regex()
        .captures(url)
        .ok_or_else(|| anyhow::anyhow!("No regex match found"))?
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Match not exists"))?;

    Ok(matched_task.as_str())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_returns_the_right_task_id() -> anyhow::Result<()> {
        let fixtures = [
            [
                "https://some-domain.atlassian.net/browse/some-task-id",
                "some-task-id",
            ],
            [
                "some-domain.atlassian.net/browse/some-other-task-id",
                "some-other-task-id",
            ],
            ["a.atlassian.net/browse/1234", "1234"],
        ];

        for fixture in fixtures.iter() {
            let task_id = get_jira_task_id_from_url(fixture[0])?;

            assert_eq!(task_id, fixture[1]);
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
            let result = get_jira_task_id_from_url(fixture);

            let error = result.unwrap_err();

            assert_eq!(error.to_string(), "No regex match found".to_string());
        }
    }
}
