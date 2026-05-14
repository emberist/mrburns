use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

use crate::constants::{
    CONFIG_FILE_NAME, DEFAULT_MR_TEMPLATE_PATH, TASK_ACTIONS_REF, TASK_ID_REF, TASK_TITLE_REF,
    TASK_TYPE_REF, TASK_URL_REF,
};

#[derive(Debug, Clone, Copy)]
pub enum ConfigScope {
    Project,
    Global,
}

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

fn project_config_path() -> PathBuf {
    PathBuf::from(CONFIG_FILE_NAME)
}

fn global_config_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;

    Some(base.join("mrburns").join(CONFIG_FILE_NAME))
}

fn scope_path(scope: ConfigScope) -> Option<PathBuf> {
    match scope {
        ConfigScope::Project => Some(project_config_path()),
        ConfigScope::Global => global_config_path(),
    }
}

fn read_json(path: &Path) -> Option<Value> {
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn merge_json(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (key, overlay_value) in overlay_map {
                merge_json(base_map.entry(key).or_insert(Value::Null), overlay_value);
            }
        }
        (slot, overlay_value) => {
            *slot = overlay_value;
        }
    }
}

impl Config {
    pub fn path(scope: ConfigScope) -> Option<PathBuf> {
        scope_path(scope)
    }

    pub fn exists(scope: ConfigScope) -> bool {
        scope_path(scope).is_some_and(|p| p.exists())
    }

    pub fn read() -> Self {
        let mut merged =
            serde_json::to_value(Self::default()).expect("default Config must serialize to JSON");

        if let Some(path) = global_config_path() {
            if let Some(value) = read_json(&path) {
                merge_json(&mut merged, value);
            }
        }

        if let Some(value) = read_json(&project_config_path()) {
            merge_json(&mut merged, value);
        }

        serde_json::from_value(merged).unwrap_or_default()
    }

    pub fn write(config: Self, scope: ConfigScope) -> anyhow::Result<()> {
        let path = scope_path(scope)
            .ok_or_else(|| anyhow::anyhow!("unable to resolve config path for the given scope"))?;

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;

        serde_json::to_writer_pretty(&mut file, &config)?;

        Ok(())
    }
}
