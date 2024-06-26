use clap::Parser;
use cli::{Cli, Commands};
use cliclack::{confirm, intro, log, outro};
use commands::{branches, browse, mr, start, wizard::start_config_wizard};
use std::process::{self, Command, Stdio};
use utils::get_latest_version;

mod cli;
mod commands;
mod config;
mod constants;
mod git;
mod repo_connectors;
mod strings;
mod task_connectors;
mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    intro(format!("mrburns v{}", VERSION)).unwrap();

    let maybe_latest_version = get_latest_version()
        .await
        .ok()
        .map(|version| version.to_string());

    if let Some(latest_version) = maybe_latest_version {
        if VERSION != latest_version {
            log::warning(format!(
                "A new version of mrburns is available: v{}. Run `cargo install mrburns --git https://github.com/emberist/mrburns` to update.",
                latest_version
            ))
            .unwrap();

            let should_update = confirm("Do you want to install it?").interact().unwrap();

            if should_update {
                Command::new("cargo")
                    .args([
                        "install",
                        "mrburns",
                        "--git",
                        "https://github.com/emberist/mrburns",
                    ])
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Something went wrong while trying to install mrburns");
            }
        }
    }

    let result = match &cli.command {
        Commands::Start(args) => start::start_task(args).await,
        Commands::Mr(args) => mr::create_mr(args).await,
        Commands::Browse => browse::browse(),
        Commands::List => branches::branches(),
        Commands::Config(args) => start_config_wizard(args),
    };

    if result.is_err() {
        let error = result.unwrap_err();

        if format!("{}", error.root_cause()).eq("operation interrupted") {
            process::exit(0);
        }

        log::error(&error).unwrap();

        outro("Done with errors.").unwrap();
        process::exit(1);
    } else {
        outro("Done.").unwrap();
    }
}
