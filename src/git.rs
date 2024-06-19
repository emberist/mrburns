use std::process::{Command, Stdio};

use anyhow::bail;

pub struct GitConfig {}

impl GitConfig {
    pub fn read(key: &str) -> anyhow::Result<String> {
        let git_remote_output = Command::new("git")
            .args(["config", "--get", key])
            .stdout(Stdio::piped())
            .output()?;

        let key = String::from_utf8(git_remote_output.stdout)?.replace('\n', "");

        if key.is_empty() {
            bail!("Config key with no content {}", key);
        }

        Ok(key)
    }

    pub fn write(key: &str, value: &str) -> anyhow::Result<()> {
        let cmd = Command::new("git")
            .args(["config", key, value])
            .stdout(Stdio::piped())
            .status()?;

        if !cmd.success() {
            anyhow::bail!("Error writing config key {}", key);
        }

        Ok(())
    }
}

pub struct GitBranch {}

impl GitBranch {
    pub fn create(name: &str) -> anyhow::Result<()> {
        let output: std::process::Output = Command::new("git")
            .args(["switch", "-c", name])
            .stdout(Stdio::piped())
            .output()?;

        let error = String::from_utf8(output.stderr)?;

        if error.contains("fatal") {
            anyhow::bail!("An error occurred creating branch {}", name);
        }

        Ok(())
    }

    pub fn default() -> anyhow::Result<String> {
        let output = Command::new("git")
            .args(["symbolic-ref", "refs/remotes/origin/HEAD", "--short"])
            .stdout(Stdio::piped())
            .output()?;

        let branch_name = String::from_utf8(output.stdout)?
            .replace('\n', "")
            .split('/')
            .last()
            .ok_or_else(|| anyhow::anyhow!("Cannot get default branch name"))?
            .to_string();

        Ok(branch_name)
    }

    pub fn current() -> anyhow::Result<String> {
        let output = Command::new("git")
            .args(["branch", "--show-current"])
            .stdout(Stdio::piped())
            .output()?;

        let branch_name = String::from_utf8(output.stdout)?.replace('\n', "");

        Ok(branch_name)
    }

    pub fn all() -> anyhow::Result<Vec<String>> {
        let output = Command::new("git")
            .args(["branch", "--list"])
            .stdout(Stdio::piped())
            .output()?;

        let branches = String::from_utf8(output.stdout)?
            .split('\n')
            .map(|s| s.trim().replace("* ", "").to_string())
            .collect();

        Ok(branches)
    }
}
