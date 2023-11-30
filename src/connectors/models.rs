use crate::strings::slugify::slugify;

#[derive(Debug, Clone)]
pub enum TaskConnector {
    Jira {
        api_base_url: String,
        task_id: String,
    },
    Asana(String),
}

pub struct TaskInfo {
    pub connector: TaskConnector,
    pub name: String,
}

impl TaskInfo {
    pub fn sanitized_name(&self) -> String {
        let connector = self.connector.to_owned();

        match connector {
            TaskConnector::Asana(_) => self.name.to_owned(),
            TaskConnector::Jira { task_id, .. } => format!("{}-{}", task_id, slugify(&self.name)),
        }
    }
}

pub trait TaskConnectorTrait {
    fn get_info(&self, connector: TaskConnector) -> TaskInfo;
}
