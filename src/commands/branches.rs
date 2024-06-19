use crate::{
    git::{GitBranch, GitConfig},
    utils::get_task_url_config_key,
};

use cliclack::{log, note};

pub fn branches() -> anyhow::Result<()> {
    let branches = GitBranch::all()?;

    let mrburns_branches: Vec<(String, String)> = branches
        .into_iter()
        .filter_map(|b| {
            let config_key = get_task_url_config_key(&b);
            let url = GitConfig::read(&config_key).ok();

            if url.is_none() {
                return None;
            }

            Some((b, url.unwrap()))
        })
        .collect();

    if mrburns_branches.is_empty() {
        let _ = log::warning(
            "No active mrburns branches found. Start a new task with `mrburns start <task_url>`",
        );
    } else {
        let note_title = format!("Found {} mrburns branches!", mrburns_branches.len());

        let urls: Vec<String> = mrburns_branches
            .iter()
            .map(|(_, url)| String::from(url))
            .collect();

        note(note_title, urls.join("\n"))?;
    }

    Ok(())
}
