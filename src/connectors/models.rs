use crate::strings::slugify;

#[derive(Debug, Clone)]

pub enum TaskConnector {
    Jira {
        api_base_url: String,
        task_id: String,
    },
    Asana(String),
}

pub struct TaskInfo {
    pub url: String,
    pub connector: TaskConnector,
    pub name: String,
}

impl TaskInfo {
    pub fn sanitized_name(&self) -> String {
        let connector = self.connector.to_owned();
        let normalized_task_name = slugify(&self.name);

        match connector {
            TaskConnector::Asana(_) => normalized_task_name,
            TaskConnector::Jira(task_id) => format!("{}-{}", task_id, normalized_task_name),
        }
    }
}

pub trait TaskConnectorTrait {
    fn get_info(&self, connector: TaskConnector, url: &str) -> TaskInfo;
}

#[derive(Debug, PartialEq)]
pub enum RepoProvider {
    Github,
    Gitlab,
    Bitbucket,
}

#[derive(Debug)]
pub struct RepoInfo {
    pub provider: RepoProvider,
    pub project: String,
}
