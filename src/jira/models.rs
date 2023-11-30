use serde::Deserialize;

use crate::connectors::models::{TaskConnector, TaskConnectorTrait, TaskInfo};

#[derive(Deserialize, Debug)]
pub struct Fields {
    pub summary: String,
}

#[derive(Deserialize, Debug)]
pub struct JiraTask {
    pub fields: Fields,
    pub key: String,
}

impl TaskConnectorTrait for JiraTask {
    fn get_info(&self, connector: TaskConnector) -> TaskInfo {
        TaskInfo {
            connector,
            name: self.fields.summary.to_owned(),
        }
    }
}
