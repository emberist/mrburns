use crate::{config::Config, connectors::models::TaskInfo, git::GitBranch};

fn stringify_query_params(params: Vec<(&str, &str)>) -> String {
    let query_params_vector: Vec<String> = params
        .iter()
        .map(|(key, value)| format!("{}={}", key, urlencoding::encode(value)))
        .collect();

    query_params_vector.join("&")
}

pub struct Params {
    pub project_id: String,
    pub title: String,
    pub task_url: String,
}

fn get_github_pull_request_creation_url(params: Params) -> anyhow::Result<String> {
    let Params {
        project_id,
        task_url,
        title,
    } = params;

    let base_branch = Config::read()?.base_branch;
    let current_branch = GitBranch::current()?;

    Ok(format!(
        "https://github.com/{}/compare/{}...{}?{}",
        project_id,
        base_branch,
        current_branch,
        stringify_query_params(vec![
            ("title", &title),
            (
                "body",
                format!(r"### Changes\n- [x] {}\n\n---\n\n{}", title, task_url).as_str()
            )
        ])
    ))
}

pub fn open_github_pull_request_creation_url(
    project_id: &str,
    task_info: &TaskInfo,
) -> anyhow::Result<String> {
    let url = get_github_pull_request_creation_url(Params {
        project_id: project_id.to_owned(),
        title: task_info.name.to_owned(),
        task_url: task_info.url.to_owned(),
    })?;

    open::that(&url)?;

    Ok(url)
}
