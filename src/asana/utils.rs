use regex::Regex;

pub fn get_asana_task_url_regex() -> Regex {
    Regex::new(r"app\.asana\.com\/(?:\d+)\/(?:\d+)\/(\d+)").unwrap()
}

pub fn get_asana_task_id_from_url(url: &str) -> anyhow::Result<&str> {
    let matched_task = get_asana_task_url_regex()
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
                "https://app.asana.com/0/0/1200371619465230/f",
                "1200371619465230",
            ],
            ["app.asana.com/0/0/1200371619465230/f", "1200371619465230"],
            ["app.asana.com/0/0/1200371619465230", "1200371619465230"],
            [
                "app.asana.com/0/1200245555103937/1200371619465230",
                "1200371619465230",
            ],
        ];

        for fixture in fixtures.iter() {
            let task_id = get_asana_task_id_from_url(fixture[0])?;

            assert_eq!(task_id, fixture[1]);
        }

        Ok(())
    }

    #[test]
    fn it_fails_getting_task_id() {
        let fixtures = [
            "https://app.foobar.com/0/0/1200371619465230/f",
            "foobar.asana.com/0/0/1200371619465230/f",
            "foobar",
            "1200371619465230",
        ];

        for fixture in fixtures.iter() {
            let result = get_asana_task_id_from_url(fixture);

            let error = result.unwrap_err();

            assert_eq!(error.to_string(), "No regex match found".to_string());
        }
    }
}
