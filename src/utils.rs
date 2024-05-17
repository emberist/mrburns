use crate::git::{GitBranch, GitConfig};

pub fn get_task_url_config_key(branch_name: &str) -> String {
    format!("branch.{}.task-url", branch_name)
}

pub fn get_current_task_url() -> anyhow::Result<String> {
    let current_branch_name = GitBranch::current()?;

    let task_url_config_key = get_task_url_config_key(&current_branch_name);

    GitConfig::read(&task_url_config_key)
}
