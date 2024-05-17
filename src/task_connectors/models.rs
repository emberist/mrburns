use crate::strings::slugify;

pub enum ConnectorType {
    Asana,
    Github,
    Jira,
}
pub trait BaseTask {
    fn get_details(&self) -> TaskDetails;
}

pub struct TaskDetails {
    pub connector: ConnectorType,
    pub name: String,
    pub id: String,
}

impl TaskDetails {
    pub fn sanitized_name(&self) -> String {
        let normalized_task_name = slugify(&self.name);

        match self.connector {
            ConnectorType::Asana => normalized_task_name,
            ConnectorType::Github => normalized_task_name,
            ConnectorType::Jira => {
                format!("{}-{}", self.id, normalized_task_name)
            }
        }
    }
}
