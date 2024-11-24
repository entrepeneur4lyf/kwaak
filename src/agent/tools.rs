#![allow(dead_code)]
use std::sync::Arc;

use anyhow::Result;
use swiftide::{
    chat_completion::{errors::ToolError, ToolOutput},
    traits::{AgentContext, Command, Output},
};
use swiftide_macros::{tool, Tool};

use crate::git::github::GithubSession;

static MAIN_BRANCH_CMD: &str = "git remote show origin | sed -n '/HEAD branch/s/.*: //p'";

#[tool(
    description = "Reads file content",
    param(name = "file_name", description = "Full path of the file")
)]
pub async fn read_file(
    context: &dyn AgentContext,
    file_name: &str,
) -> Result<ToolOutput, ToolError> {
    let cmd = Command::Shell(format!("cat {file_name}"));

    let output = context.exec_cmd(&cmd).await?;

    Ok(output.into())
}

#[tool(
    description = "Write a file",
    param(name = "file_name", description = "Full path of the file"),
    param(name = "content", description = "Content to write to the file")
)]
pub async fn write_file(
    context: &dyn AgentContext,
    file_name: &str,
    content: &str,
) -> Result<ToolOutput, ToolError> {
    let heredoc = format!("<<HERE\n{content}\nHERE");
    let cmd = Command::Shell(format!("echo {heredoc} > {file_name}"));

    let output = context.exec_cmd(&cmd).await?;

    Ok(output.into())
}

#[tool(
    description = "Searches for a file",
    param(name = "file_name", description = "Partial or full name of the file")
)]
pub async fn search_file(
    context: &dyn AgentContext,
    file_name: &str,
) -> Result<ToolOutput, ToolError> {
    let cmd = Command::Shell(format!("find . -name '*{file_name}*'"));
    let output = context.exec_cmd(&cmd).await?;

    Ok(output.into())
}

#[tool(
    description = "Invoke a git command on the current repository",
    param(name = "command", description = "Git sub-command to run")
)]
pub async fn git(context: &dyn AgentContext, command: &str) -> Result<ToolOutput, ToolError> {
    let cmd = Command::Shell(format!("git {command}"));
    let output = context.exec_cmd(&cmd).await?;

    Ok(output.into())
}

#[derive(Tool, Clone, Debug)]
#[tool(
    description = "Creates a pull request on Github with the current branch onto the main branch. Pushes the current branch to the remote repository.",
    param(name = "title", description = "Title of the pull request"),
    param(name = "pull_request_body", description = "Body of the pull request")
)]
pub struct CreatePullRequest {
    github_session: Arc<GithubSession>,
}

impl<'a> CreatePullRequest {
    pub fn new(github_session: &Arc<GithubSession>) -> Self {
        Self {
            github_session: Arc::clone(github_session),
        }
    }

    async fn create_pull_request(
        &self,
        context: &dyn AgentContext,
        title: &str,
        pull_request_body: &str,
    ) -> Result<ToolOutput, ToolError> {
        // Push the current branch first
        let cmd = Command::Shell("git push origin HEAD".to_string());
        context.exec_cmd(&cmd).await?;

        // Get the current branch and main branch, then create the pull request
        // TODO: Illustrates that current cmd output is too involved, should use Rust results
        // properly instead
        // Also, if input is shell, you kinda always expect a shell output? Maybe generics can
        // solve this better
        let current_branch = context
            .exec_cmd(&Command::shell("git branch --show-current"))
            .await?;

        let current_branch = match current_branch {
            Output::Shell {
                stdout, success, ..
            } if success => stdout,
            Output::Shell {
                stderr, success, ..
            } if !success => {
                return Err(anyhow::anyhow!("Failed to get current branch: {}", stderr).into())
            }
            _ => {
                return Err(
                    anyhow::anyhow!("Unexpected output from git branch --show-current").into(),
                )
            }
        };

        // Main branch leaks from github session intentionally as it would be cool to use other
        // branches as well. However, if that's never the case, just simplify the api.
        let pull_request = self
            .github_session
            .create_pull_request(
                current_branch,
                &self.github_session.main_branch(),
                title,
                pull_request_body,
            )
            .await?;

        Ok(ToolOutput::Text(format!(
            "Created a pull request at {}",
            pull_request.url
        )))
    }
}

#[derive(Tool, Clone, Debug)]
#[tool(description = "Runs tests")]
pub struct RunTests {
    pub test_command: String,
}

impl RunTests {
    pub fn new(test_command: String) -> Self {
        Self { test_command }
    }

    async fn run_tests(&self, context: &dyn AgentContext) -> Result<ToolOutput, ToolError> {
        let cmd = Command::Shell(self.test_command.clone());
        let output = context.exec_cmd(&cmd).await?;

        Ok(output.into())
    }
}
// read file
// write file
// search file
// run tests
