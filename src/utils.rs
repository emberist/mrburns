use anyhow::{bail, Context, Result};
use chrono::{TimeZone, Utc};
use reqwest::header::USER_AGENT;
use reqwest::Client;
use semver::Version;
use serde::{de, Deserialize};

use crate::git::{GitBranch, GitConfig};

pub fn get_task_url_config_key(branch_name: &str) -> String {
    format!("branch.{}.task-url", branch_name)
}

pub fn get_current_task_url() -> anyhow::Result<String> {
    let current_branch_name = GitBranch::current()?;

    let task_url_config_key = get_task_url_config_key(&current_branch_name);

    GitConfig::read(&task_url_config_key)
}

async fn fetch_versions() -> Result<Vec<Version>> {
    #[derive(Deserialize)]
    struct Release {
        #[serde(rename = "name", deserialize_with = "version_deserializer")]
        version: Version,
    }

    fn version_deserializer<'de, D>(deserializer: D) -> Result<Version, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s: &str = de::Deserialize::deserialize(deserializer)?;
        Version::parse(s.trim_start_matches('v')).map_err(de::Error::custom)
    }

    let response = Client::new()
        .get("https://api.github.com/repos/emberist/mrburns/tags")
        .header(
            USER_AGENT,
            "mrburns https://github.com/emberist/mrburns/mrburns",
        )
        .send()
        .await
        .context("Failed to get tags")?;

    if response.status().is_success() {
        let releases: Vec<Release> = response.json().await?;
        let versions = releases.into_iter().map(|r| r.version).collect();

        Ok(versions)
    } else {
        let reset_time_header = response
            .headers()
            .get("X-RateLimit-Reset")
            .map_or("unknown", |v| v.to_str().unwrap());

        let t = Utc.timestamp_opt(reset_time_header.parse::<i64>().unwrap(), 0);

        let reset_time = t
            .single()
            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "unknown".to_string());

        bail!(
            "GitHub API rate limit exceeded. Try again after {} UTC.",
            reset_time
        )
    }
}

pub async fn get_latest_version() -> Result<Version> {
    fetch_versions()
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("First version not found"))
}
