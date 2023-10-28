use clap::Parser;
use cli::{Cli, Commands};
use cliclack::{log, outro};
use commands::{browse, mr, start, wizard::start_config_wizard};
use std::process;

mod cli;
mod commands;
mod config;
mod git;
mod gitlab;
mod jira;
mod strings;
mod utils;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let result = match &cli.command {
        Commands::Start(args) => start::start_task(args).await,
        Commands::Mr(args) => mr::create_mr(args).await,
        Commands::Browse => browse::browse(),
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
