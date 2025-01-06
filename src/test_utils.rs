extern crate tempfile;

use crate::config::Config;
use crate::repository::Repository;

pub struct TestGuard {
    pub tempdir: tempfile::TempDir,
}

impl From<&Config> for Repository {
    fn from(config: &Config) -> Self {
        // Adjust the dereferencing of config
        Repository::from_config(*config.clone())
    }
}

pub fn test_repository(config: &Config) -> (Repository, TestGuard) {
    let tempdir = tempfile::TempDir::new().expect("Failed to create tempdir");
    let repo_path = tempdir.path().join("repo");
    std::fs::create_dir(&repo_path).expect("Failed to create repo directory");
    let repository = Repository::from(config);
    (repository, TestGuard { tempdir })
}
