use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::Path};

use crate::constants::{
    CONFIG_FILE_NAME, DEFAULT_MR_TEMPLATE_PATH, TASK_ACTIONS_REF, TASK_ID_REF, TASK_TITLE_REF,
    TASK_TYPE_REF, TASK_URL_REF,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchPrefixes {
    pub feature: String,
    pub release: String,
    pub bugfix: String,
    pub chore: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MrConfig {
    #[serde(default)]
    pub default_draft: bool,
    #[serde(default)]
    pub title_template: String,
    #[serde(default)]
    pub description_template_path: String,
    #[serde(default)]
    pub description_template: Vec<String>,
}

impl Default for MrConfig {
    fn default() -> Self {
        Self {
            default_draft: false,
            title_template: format!("{}/{}/{}", TASK_ID_REF, TASK_TYPE_REF, TASK_TITLE_REF),
            description_template_path: DEFAULT_MR_TEMPLATE_PATH.to_string(),
            description_template: vec![
                "### Changes".to_string(),
                format!("- [x] [{}]({})", TASK_TITLE_REF, TASK_URL_REF),
                "".to_string(),
                "---".to_string(),
                TASK_ACTIONS_REF.to_string(),
            ],
        }
    }
}

impl Default for BranchPrefixes {
    fn default() -> Self {
        Self {
            feature: "feat".to_string(),
            release: "release".to_string(),
            bugfix: "bugfix".to_string(),
            chore: "chore".to_string(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub mr: MrConfig,
    #[serde(default)]
    pub branch_prefixes: BranchPrefixes,
}

impl Config {
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

        let config: Config = serde_json::from_str(&config_file_content).unwrap_or(default_config);

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
