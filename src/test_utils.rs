extern crate tempfile;

use crate::config::Config;
use crate::repository::Repository;

pub struct TestGuard {
    pub tempdir: tempfile::TempDir,
}

impl From<&Config> for Repository {
    fn from(config: &Config) -> Self {
        // Implement the logic to create a Repository from a Config
        // The repository struct needs to be updated with the correct initialization logic
        Repository::from_config(config) // Ensure this function is correctly implemented in `Repository`
    }
}

pub fn test_repository(config: &Config) -> (Repository, TestGuard) {
    let tempdir = tempfile::TempDir::new().expect("Failed to create tempdir");
    let repo_path = tempdir.path().join("repo");
    std::fs::create_dir(&repo_path).expect("Failed to create repo directory");
    let repository = Repository::from(config);
    (repository, TestGuard { tempdir })
}
