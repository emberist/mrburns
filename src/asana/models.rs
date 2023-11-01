use serde::Deserialize;

use crate::connectors::models::{Task, TaskConnector, TaskDetails};

#[derive(Debug, Deserialize)]
pub struct AsanaTask {
    pub gid: String,
    pub name: String,
}

impl Task for AsanaTask {
    fn info(&self, connector: TaskConnector) -> TaskDetails {
        TaskDetails {
            connector,
            name: self.name.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub data: T,
}
