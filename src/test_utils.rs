extern crate tempfile;

use crate::config::Config;
use crate::repository::Repository;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct TestGuard {
    pub tempdir: TempDir,
}

pub fn test_repository(config: &Config) -> (Repository, TestGuard) {
    let tempdir = TempDir::new().expect("Failed to create tempdir");
    let repo_path = tempdir.path().join("repo");
    std::fs::create_dir(&repo_path).expect("Failed to create repo directory");
    let repository =
        Repository::from_config(config.into()).expect("Failed to create test repository");
    (repository, TestGuard { tempdir })
}
