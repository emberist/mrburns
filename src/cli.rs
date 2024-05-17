use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;
use url::Url;

use crate::VERSION;

#[derive(Parser)]
#[command(name = "mrburns")]
#[command(author = "emberist <emberistemac@proton.me>")]
#[command(version = VERSION)]
#[command(about = "Your friendly neighborhood tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum TaskType {
    Bugfix,
    Chore,
    Feature,
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskType::Bugfix => write!(f, "bugfix"),
            TaskType::Chore => write!(f, "chore"),
            TaskType::Feature => write!(f, "feature"),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Create a new branch from the given task url")]
    Start(StartArgs),
    #[command(about = "Open the link of the current task in the browser")]
    Browse,
    #[command(about = "Helps opening a MR for the current branch")]
    Mr(MrArgs),
    #[command(about = "Set mrburns configuration")]
    Config(ConfigArgs),
}

#[derive(Args, Debug)]
pub struct StartArgs {
    pub link: Url,
    #[arg(value_enum, default_value_t = TaskType::Feature, long = "type", short)]
    pub task_type: TaskType,
    #[arg(long, default_value_t = false)]
    pub dry: bool,
}

#[derive(Args, Debug)]
pub struct MrArgs {
    #[arg(long = "base", short = 'b', name = "base-branch")]
    pub base_branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(long, default_value_t = false)]
    pub force: bool,

    #[arg(long, default_value_t = false)]
    pub get: bool,
}
