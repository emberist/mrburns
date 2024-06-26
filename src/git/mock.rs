use super::adapter::GitClientAdapter;

pub struct GitClientMock {}

impl GitClientAdapter for GitClientMock {
    fn is_clean(&self) -> anyhow::Result<bool> {
        Ok(true)
    }

    fn switch(&self, _: &str, _: bool) -> anyhow::Result<()> {
        Ok(())
    }

    fn default_branch(&self) -> anyhow::Result<String> {
        Ok("main".to_string())
    }

    fn current_branch(&self) -> anyhow::Result<String> {
        Ok("feat/some-cool-feature".to_string())
    }

    fn all_branches(&self) -> anyhow::Result<Vec<String>> {
        Ok(vec![
            "feat/some-cool-feature".to_string(),
            "fix/some-fix-branch".to_string(),
        ])
    }

    fn read_config(&self, _: &str) -> anyhow::Result<String> {
        Ok("https://github.com/some-cool-repo/issues/0".to_string())
    }

    fn write_config(&self, _: &str, _: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
