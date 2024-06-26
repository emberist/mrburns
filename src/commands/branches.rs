use crate::{
    git::{adapter::GitClientAdapter, client::GitClient},
    utils::get_task_url_config_key,
};

use cliclack::{log, select};

pub fn branches() -> anyhow::Result<()> {
    let git_client = GitClient {};

    if !git_client.is_clean()? {
        log::error(
            "You have uncommitted changes. Please commit or stash them before switching branches.",
        )?;

        return Ok(());
    }

    let branches = git_client.all_branches()?;

    let current_branch = git_client.current_branch()?;

    let mrburns_branches: Vec<(String, String)> = branches
        .into_iter()
        .filter_map(|b| {
            let config_key = get_task_url_config_key(&b);
            let url = git_client.read_config(&config_key).ok();

            if b == current_branch || url.is_none() {
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

        log::success(note_title)?;

        let urls: Vec<(String, String, String)> = mrburns_branches
            .iter()
            .map(|(branch, url)| (String::from(branch), String::from(url), branch.to_string()))
            .collect();

        let selected_branch = select("To which task do you want to switch?")
            .items(&urls)
            .interact()?;

        git_client.switch(&selected_branch, false)?;

        log::info(format!("Switched to branch {}", selected_branch))?;
    }

    Ok(())
}
