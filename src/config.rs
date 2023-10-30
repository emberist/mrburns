use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::Path};

const CONFIG_FILE_NAME: &str = "mrburns.config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonConfig {
    pub main_branch: String,
    pub create_draft_mr: Option<bool>,
    pub jira_api_base_url: Option<String>,
}

pub struct Config {
    pub main_branch: String,
    pub create_draft_mr: bool,
    pub jira_api_base_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main_branch: "master".to_string(),
            create_draft_mr: true,
            jira_api_base_url: None,
        }
    }
}

impl Config {
    fn from_json(json_config: JsonConfig) -> Self {
        let default_config = Self::default();

        Self {
            main_branch: json_config.main_branch,
            create_draft_mr: json_config
                .create_draft_mr
                .unwrap_or(default_config.create_draft_mr),
            jira_api_base_url: json_config.jira_api_base_url,
        }
    }

    pub fn to_json(&self) -> JsonConfig {
        JsonConfig {
            main_branch: self.main_branch.clone(),
            create_draft_mr: Some(self.create_draft_mr),
            jira_api_base_url: self.jira_api_base_url.clone(),
        }
    }

    pub fn exists() -> bool {
        Path::new(CONFIG_FILE_NAME).exists()
    }

    pub fn read() -> anyhow::Result<Self> {
        if !Self::exists() {
            return Ok(Self::default());
        }

        let config_file_content = std::fs::read_to_string(CONFIG_FILE_NAME)?;
        let json_config: JsonConfig = serde_json::from_str(&config_file_content)?;

        Ok(Self::from_json(json_config))
    }

    pub fn write(config: Self) -> anyhow::Result<()> {
        let json_config = config.to_json();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(CONFIG_FILE_NAME)?;

        // let mut file = File::create(CONFIG_FILE_NAME)?;

        serde_json::to_writer_pretty(&mut file, &json_config)?;

        Ok(())
    }
}
