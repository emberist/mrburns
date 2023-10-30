use serde::Deserialize;

use crate::connectors::models::{TaskConnector, TaskConnectorTrait, TaskInfo};

#[derive(Debug, Deserialize)]
pub struct AsanaTask {
    pub gid: String,
    pub name: String,
}

impl TaskConnectorTrait for AsanaTask {
    fn get_info(&self) -> TaskInfo {
        TaskInfo {
            connector: TaskConnector::Asana(self.gid.to_owned()),
            name: self.name.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub data: T,
}
