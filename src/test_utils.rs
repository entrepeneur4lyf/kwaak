use crate::config::{
    defaults::{default_main_branch, default_project_name},
    Config, GithubConfiguration,
};
use crate::repository::Repository;

// Define TestGuard as a struct with a tempdir field
pub struct TestGuard {
    pub tempdir: tempfile::TempDir, // Assuming tempfile crate is used for temporary directories
}

// Function to create a test repository and return it along with a TestGuard
pub fn test_repository() -> (Repository, TestGuard) {
    // Implement the setup logic for a test repository
    // and create a TestGuard with a tempfile::TempDir
    let tempdir = tempfile::TempDir::new().expect("Failed to create tempdir");

    // Create a default config or adjust as needed
    let config = Config {
        project_name: default_project_name(),
        github: GithubConfiguration {
            repository: "test-repo".into(),
            owner: "test-owner".into(),
            main_branch: default_main_branch(),
            token: None,
        },
        ..Default::default() // Ensure Config has a Default implementation or provide required fields
    };

    // Use from_config instead of new
    let repository = Repository::from_config(config);

    (repository, TestGuard { tempdir })
}
