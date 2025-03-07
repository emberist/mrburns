use cliclack::{
    confirm, input,
    log::{self, info},
};

use crate::{
    cli::ConfigArgs,
    config::{Branches, MrConfig, MrburnsConfig, Prefix},
};

pub fn start_config_wizard(args: &ConfigArgs) -> anyhow::Result<()> {
    info("Configuration started")?;

    if args.get {
        let config = MrburnsConfig::read();

        log::success(serde_json::to_string_pretty(&config)?)?;

        return Ok(());
    }

    if MrburnsConfig::exists() && !args.force {
        log::warning(
            "Configuration already exists. Skipping wizard. Use the --force option to override it",
        )?;

        return Ok(());
    }

    if args.force && MrburnsConfig::exists() {
        let should_continue =
            confirm("You are overriding the current configuration. Do you want to preceed?")
                .interact()?;

        if !should_continue {
            return Ok(());
        }
    }

    let default_draft =
        confirm("Do you want the merge request created to be draft as default?").interact()?;

    let default_config = MrburnsConfig::default();

    let title_format = input("Which title template would you like to use?")
        .default_input(&default_config.mr.title_format)
        .interact()?;

    let feature = input("Which feature branch prefix would you like to use?")
        .default_input(&default_config.branches.prefixes.feature)
        .interact()?;

    let chore = input("Which chore branch prefix would you like to use?")
        .default_input(&default_config.branches.prefixes.chore)
        .interact()?;

    let bugfix = input("Which bugfix branch prefix would you like to use?")
        .default_input(&default_config.branches.prefixes.bugfix)
        .interact()?;

    let release = input("Which release branch prefix would you like to use?")
        .default_input(&default_config.branches.prefixes.release)
        .interact()?;

    MrburnsConfig::write(MrburnsConfig {
        mr: MrConfig {
            default_draft,
            title_format,
        },
        branches: Branches {
            include_task_identifier: true,

            prefixes: Prefix {
                bugfix,
                chore,
                feature,
                release,
            },
        },
    })?;

    Ok(())
}
