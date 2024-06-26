use cliclack::{
    confirm, input,
    log::{self, info},
};

use crate::{
    cli::ConfigArgs,
    config::{BranchPrefixes, Config, MrConfig},
    constants::DEFAULT_MR_TEMPLATE_PATH,
};

pub fn start_config_wizard(args: &ConfigArgs) -> anyhow::Result<()> {
    info("Configuration started")?;

    if args.get {
        let config = Config::read();

        log::success(format!(
            "Here your config: {}",
            serde_json::to_string_pretty(&config)?
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

    let default_draft =
        confirm("Do you want the merge request created to be draft as default?").interact()?;

    let default_config = Config::default();

    let title_template = input("Which title template would you like to use?")
        .default_input(&default_config.mr.title_template)
        .interact()?;

    let description_template_path =
        input("Which description template would you like to use? (Leave empty to skip)")
            .default_input(DEFAULT_MR_TEMPLATE_PATH)
            .interact()?;

    let feature = input("Which feature branch prefix would you like to use?")
        .default_input(&default_config.branch_prefixes.feature)
        .interact()?;

    let chore = input("Which chore branch prefix would you like to use?")
        .default_input(&default_config.branch_prefixes.chore)
        .interact()?;

    let bugfix = input("Which bugfix branch prefix would you like to use?")
        .default_input(&default_config.branch_prefixes.bugfix)
        .interact()?;

    let release = input("Which release branch prefix would you like to use?")
        .default_input(&default_config.branch_prefixes.release)
        .interact()?;

    Config::write(Config {
        mr: MrConfig {
            default_draft,
            title_template,
            description_template_path,
        },
        branch_prefixes: BranchPrefixes {
            bugfix,
            chore,
            feature,
            release,
        },
    })?;

    Ok(())
}
