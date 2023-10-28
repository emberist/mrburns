use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Domain {
    Github,
    Gitlab,
    Bitbucket,
}

#[derive(Debug)]
pub struct RepoUrlInfo {
    pub domain: Domain,
    pub project: String,
}

pub fn parse_repo_url(url: &str) -> anyhow::Result<RepoUrlInfo> {
    let captures = Regex::new(
        r"^(?:git@|https:\/\/)(gitlab\.com|github\.com|bitbucket\.org)(?::|\/)(.+)\.git$",
    )?
    .captures(url)
    .ok_or_else(|| anyhow::anyhow!("No regex match found"))?;

    let url_domain = captures.get(1).expect("No  match found").as_str();

    let domain = match url_domain {
        "github.com" => Domain::Github,
        "gitlab.com" => Domain::Gitlab,
        "bitbucket.org" => Domain::Bitbucket,
        _ => panic!("Unknown url domain"),
    };

    Ok(RepoUrlInfo {
        domain,
        project: String::from(captures.get(2).expect("No  match found").as_str()),
    })
}

pub fn get_task_url_config_key(branch_name: &str) -> String {
    format!("branch.{}.task-url", branch_name)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_returns_right_domain() -> anyhow::Result<()> {
        let RepoUrlInfo { domain, project } = parse_repo_url("https://github.com/org/repo.git")?;

        let RepoUrlInfo {
            domain: domain2,
            project: project2,
        } = parse_repo_url("https://gitlab.com/some_org/some_repo.git")?;

        let RepoUrlInfo {
            domain: domain3,
            project: project3,
        } = parse_repo_url("https://bitbucket.org/fake_org/repo.git")?;

        let RepoUrlInfo {
            domain: domain4,
            project: project4,
        } = parse_repo_url("git@github.com:fake_org2/repo2.git")?;

        assert_eq!(domain, Domain::Github);
        assert_eq!(domain2, Domain::Gitlab);
        assert_eq!(domain3, Domain::Bitbucket);
        assert_eq!(domain4, Domain::Github);

        assert_eq!(project, "org/repo");
        assert_eq!(project2, "some_org/some_repo");
        assert_eq!(project3, "fake_org/repo");
        assert_eq!(project4, "fake_org2/repo2");

        Ok(())
    }

    #[test]
    fn it_fails_with_unsupported_domain() {
        let result = parse_repo_url("https://some-invalid-domain.com/org/repo.git");

        assert!(result.is_err());

        let error = result.unwrap_err();
        let root_cause = error.root_cause();

        assert_eq!(format!("{}", root_cause), "No regex match found");
    }

    #[test]
    fn it_fails_without_a_project() {
        let result = parse_repo_url("https://github.com");

        assert!(result.is_err());

        let error = result.unwrap_err();
        let root_cause = error.root_cause();

        assert_eq!(format!("{}", root_cause), "No regex match found");
    }
}