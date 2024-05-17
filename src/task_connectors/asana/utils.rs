use regex::Regex;

fn make_asana_task_url_regex() -> Regex {
    Regex::new(r"app\.asana\.com\/(?:\d+)\/(?:\d+)\/(\d+)").unwrap()
}

pub fn get_asana_task_id_from_url(url: &str) -> Option<&str> {
    make_asana_task_url_regex()
        .captures(url)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
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
            let task_id = get_asana_task_id_from_url(fixture[0]).unwrap();

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

            assert_eq!(result.is_none(), true);
        }
    }
}
