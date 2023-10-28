use cliclack::{confirm, input, intro, log, select};
use url::Url;

use crate::{cli::ConfigArgs, config::Config};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum TaskProvider {
    Asana,
    #[default]
    Jira,
    Github,
}

pub fn start_config_wizard(args: &ConfigArgs) -> anyhow::Result<()> {
    if Config::exists() && !args.force {
        log::warning(
            "Configuration already exists. Skipping wizard. Use the --force option to override it",
        )?;

        return Ok(());
    }

    intro("Starting configuration")?;

    if args.force && Config::exists() {
        let should_continue =
            confirm("You are overriding the current configuration. Do you want to preceed?")
                .interact()?;

        if !should_continue {
            return Ok(());
        }
    }

    let main_branch = input("Which is the base branch you wanna use as default?")
        .default_input("main")
        .interact()?;

    let task_provider = select("Which task provider do you use?")
        .item(TaskProvider::Jira, "Jira", "")
        .item(TaskProvider::Asana, "Asana", "")
        .item(TaskProvider::Github, "Github", "")
        .interact()?;

    let config = match task_provider {
        TaskProvider::Jira => {
            let jira_api_base_url: String = input("What's your Jira API base url?")
                .placeholder("https://yourcompany.atlassian.net")
                .validate(|input: &String| {
                    Url::parse(input)
                        .map(|_| ())
                        .map_err(|_| "Invalid URL".to_string())
                })
                .interact()?;

            Config {
                jira_api_base_url: Some(jira_api_base_url),
                ..Config::default()
            }
        }
        other_task_provider => {
            anyhow::bail!(
                "The {:?} provider is not supported yet",
                other_task_provider
            )
        }
    };

    let create_draft_mr =
        confirm("Do you want the merge request created to be draft as default?").interact()?;

    Config::write(Config {
        main_branch,
        create_draft_mr,
        ..config
    })?;

    Ok(())
}
