use serde::Deserialize;

use crate::connectors::models::{Task, TaskConnector, TaskDetails};

#[derive(Deserialize, Debug)]
pub struct Fields {
    pub summary: String,
}

#[derive(Deserialize, Debug)]
pub struct JiraTask {
    pub fields: Fields,
    pub key: String,
}

impl Task for JiraTask {
    fn info(&self, connector: TaskConnector) -> TaskDetails {
        TaskDetails {
            connector,
            name: self.fields.summary.to_owned(),
        }
    }
}
