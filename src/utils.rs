use crate::{
    connectors::utils::get_task_url_config_key,
    git::{GitBranch, GitConfig},
};

pub fn get_current_task_url() -> anyhow::Result<String> {
    let current_branch_name = GitBranch::current()?;

    let task_url_config_key = get_task_url_config_key(&current_branch_name);

    GitConfig::read(&task_url_config_key)
}
