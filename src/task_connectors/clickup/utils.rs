use regex::Regex;

pub fn make_clickup_task_url_regex() -> Regex {
    Regex::new(r"app\.clickup\.com\/t\/(\w+)").unwrap()
}

pub fn get_clickup_task_info_from_url(url: &str) -> Option<&str> {
    match make_clickup_task_url_regex().captures(url) {
        Some(caps) => caps.get(1).map(|m| m.as_str()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_returns_the_right_task_id() -> anyhow::Result<()> {
        let fixtures = [
            ("https://app.clickup.com/t/869875ydy", "869875ydy"),
            ("https://app.clickup.com/t/some", "some"),
        ];

        for fixture in fixtures.iter() {
            let task_id = get_clickup_task_info_from_url(fixture.0).unwrap();

            assert_eq!(task_id, fixture.1);
        }

        Ok(())
    }

    #[test]
    fn it_fails_getting_task_id() {
        let fixtures = [
            "https://app.clickup.com/869875ydy",
            "https://clickup.com/t/869875ydy",
            "https://app.clickup.com/t/",
        ];

        for fixture in fixtures.iter() {
            let result = get_clickup_task_info_from_url(fixture);

            assert!(result.is_none());
        }
    }
}
