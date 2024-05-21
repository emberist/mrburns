use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::Path};

use crate::constants::CONFIG_FILE_NAME;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchPrefixes {
    pub feature: String,
    pub release: String,
    pub bugfix: String,
    pub chore: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonConfig {
    pub create_draft_mr: Option<bool>,
    pub branch_prefixes: Option<BranchPrefixes>,
}

pub struct Config {
    pub create_draft_mr: bool,
    pub branch_prefixes: BranchPrefixes,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            create_draft_mr: true,
            branch_prefixes: BranchPrefixes {
                bugfix: "bugfix".to_string(),
                chore: "chore".to_string(),
                feature: "feat".to_string(),
                release: "release".to_string(),
            },
        }
    }
}

impl Config {
    fn from_json(json_config: JsonConfig) -> Self {
        let default_config = Self::default();

        Self {
            create_draft_mr: json_config
                .create_draft_mr
                .unwrap_or(default_config.create_draft_mr),
            branch_prefixes: json_config
                .branch_prefixes
                .unwrap_or(default_config.branch_prefixes),
        }
    }

    pub fn to_json(&self) -> JsonConfig {
        JsonConfig {
            create_draft_mr: Some(self.create_draft_mr),
            branch_prefixes: Some(self.branch_prefixes.clone()),
        }
    }

    pub fn exists() -> bool {
        Path::new(CONFIG_FILE_NAME).exists()
    }

    pub fn read() -> Self {
        let default_config = Self::default();

        if !Self::exists() {
            return default_config;
        }

        let config_file_content =
            std::fs::read_to_string(CONFIG_FILE_NAME).unwrap_or(String::new());

        let json_config: JsonConfig =
            serde_json::from_str(&config_file_content).unwrap_or(default_config.to_json());

        Self::from_json(json_config)
    }

    pub fn write(config: Self) -> anyhow::Result<()> {
        let json_config = config.to_json();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(CONFIG_FILE_NAME)?;

        serde_json::to_writer_pretty(&mut file, &json_config)?;

        Ok(())
    }
}
