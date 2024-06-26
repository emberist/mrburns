pub trait GitClientAdapter {
    fn is_clean(&self) -> anyhow::Result<bool>;
    fn switch(&self, name: &str, create_branch: bool) -> anyhow::Result<()>;
    fn default_branch(&self) -> anyhow::Result<String>;
    fn current_branch(&self) -> anyhow::Result<String>;
    fn all_branches(&self) -> anyhow::Result<Vec<String>>;
    fn read_config(&self, key: &str) -> anyhow::Result<String>;
    fn write_config(&self, key: &str, value: &str) -> anyhow::Result<()>;
}
