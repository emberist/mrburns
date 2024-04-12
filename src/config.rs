use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::Path};

use crate::constants::CONFIG_FILE_NAME;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonConfig {
    pub create_draft_mr: Option<bool>,
}

pub struct Config {
    pub create_draft_mr: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            create_draft_mr: true,
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
        }
    }

    pub fn to_json(&self) -> JsonConfig {
        JsonConfig {
            create_draft_mr: Some(self.create_draft_mr),
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

        serde_json::to_writer_pretty(&mut file, &json_config)?;

        Ok(())
    }
}
