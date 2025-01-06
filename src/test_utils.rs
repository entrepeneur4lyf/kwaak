use crate::config::{
    defaults::{default_main_branch, default_owner_and_repo, default_project_name},
    Config,
};
use crate::{repository::Repository, templates::Templates};

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
        main_branch: default_main_branch(),
        owner_and_repo: default_owner_and_repo(),
        project_name: default_project_name(),
        ..Default::default() // Ensure Config has a Default implementation or provide required fields
    };

    // Use from_config instead of new
    let repository = Repository::from_config(config);

    (repository, TestGuard { tempdir })
}
