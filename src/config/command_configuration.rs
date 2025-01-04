//! Configuration for commands that tools can use to operate on the project.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandConfiguration {
    pub test: String,
    pub coverage: String,
    /// Optional: Lint and fix the project
    /// This command is run if any files were written to the project.
    ///
    /// i.e. in Rust `cargo clippy --fix --allow-dirty --allow-staged && cargo fmt`
    #[serde(default)]
    pub lint_and_fix: Option<String>,
}

impl Default for CommandConfiguration {
    fn default() -> Self {
        CommandConfiguration {
            test: "cargo test".to_string(),
            coverage: "cargo tarpaulin".to_string(),
            lint_and_fix: Some("cargo clippy --fix --allow-dirty --allow-staged && cargo fmt".to_string()),
        }
    }
}
