use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::Path;

use crate::constants::{
    CONFIG_FILE_NAME, TASK_ACTIONS_REF, TASK_DESCRIPTION_REF, TASK_ID_REF, TASK_TITLE_REF,
    TASK_TYPE_REF, TASK_URL_REF,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branches {
    // pub default_prefix: String,
    pub include_task_identifier: bool,
    pub prefixes: Prefix,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prefix {
    pub feature: String,
    pub release: String,
    pub bugfix: String,
    pub chore: String,
}

impl Default for Prefix {
    fn default() -> Self {
        Self {
            feature: "feat".to_string(),
            release: "release".to_string(),
            bugfix: "bugfix".to_string(),
            chore: "chore".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MrConfig {
    #[serde(default)]
    pub default_draft: bool,
    #[serde(default)]
    pub title_format: String,
}

pub fn get_default_mr_description() -> String {
    vec![
        format!("# [{}]({})", TASK_TITLE_REF, TASK_URL_REF),
        "## Description".to_string(),
        TASK_DESCRIPTION_REF.to_string(),
        "".to_string(),
        "---".to_string(),
        TASK_ACTIONS_REF.to_string(),
    ]
    .join("\n")
}

impl Default for MrConfig {
    fn default() -> Self {
        Self {
            default_draft: false,
            title_format: format!("{}/{}/{}", TASK_ID_REF, TASK_TYPE_REF, TASK_TITLE_REF),
        }
    }
}

impl Default for Branches {
    fn default() -> Self {
        Self {
            include_task_identifier: true,
            prefixes: Prefix::default(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MrburnsConfig {
    #[serde(default)]
    pub mr: MrConfig,
    #[serde(default)]
    pub branches: Branches,
}

impl MrburnsConfig {
    pub fn exists() -> bool {
        Path::new(CONFIG_FILE_NAME).exists()
    }

    pub fn read() -> Self {
        let default_config = Self::default();

        if !Self::exists() {
            return default_config;
        }

        let config_file_content = std::fs::read_to_string(CONFIG_FILE_NAME).unwrap_or_default();

        let config: MrburnsConfig =
            serde_json::from_str(&config_file_content).unwrap_or(default_config);

        config
    }

    pub fn write(config: Self) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(CONFIG_FILE_NAME)?;

        serde_json::to_writer_pretty(&mut file, &config)?;

        Ok(())
    }
}
