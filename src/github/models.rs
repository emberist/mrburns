use serde::Deserialize;

use crate::connectors::models::{Task, TaskConnector, TaskDetails};

#[derive(Debug, Deserialize)]
pub struct GithubIssue {
    title: String,
}

impl Task for GithubIssue {
    fn info(&self, connector: TaskConnector) -> TaskDetails {
        TaskDetails {
            connector,
            name: self.title.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub data: T,
}
