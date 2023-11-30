use cliclack::{confirm, intro, log};

use crate::{cli::ConfigArgs, config::Config};

pub fn start_config_wizard(args: &ConfigArgs) -> anyhow::Result<()> {
    intro("Configuration started")?;

    if args.get {
        let config = Config::read()?;

        log::success(format!(
            "Here your config: {}",
            serde_json::to_string_pretty(&config.to_json())?
        ))?;

        return Ok(());
    }

    if Config::exists() && !args.force {
        log::warning(
            "Configuration already exists. Skipping wizard. Use the --force option to override it",
        )?;

        return Ok(());
    }

    if args.force && Config::exists() {
        let should_continue =
            confirm("You are overriding the current configuration. Do you want to preceed?")
                .interact()?;

        if !should_continue {
            return Ok(());
        }
    }

    let create_draft_mr =
        confirm("Do you want the merge request created to be draft as default?").interact()?;

    Config::write(Config { create_draft_mr })?;

    Ok(())
}
