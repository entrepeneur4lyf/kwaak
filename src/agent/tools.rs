#![allow(dead_code)]
use std::sync::Arc;

use anyhow::Result;
use swiftide::{
    chat_completion::{errors::ToolError, ToolOutput},
    traits::{AgentContext, Command, CommandOutput},
};
use swiftide_macros::{tool, Tool};

use crate::git::github::GithubSession;

static MAIN_BRANCH_CMD: &str = "git remote show origin | sed -n '/HEAD branch/s/.*: //p'";

/// WARN: Experimental
#[tool(
    description = "Run any shell command in the current repository",
    param(
        name = "cmd",
        description = "The shell command, including any arguments if needed, to run"
    )
)]
pub async fn shell_command(context: &dyn AgentContext, cmd: &str) -> Result<ToolOutput, ToolError> {
    let output = context.exec_cmd(&Command::Shell(cmd.into())).await?;
    Ok(output.into())
}

#[tool(
    description = "Reads file content",
    param(name = "file_name", description = "Full path of the file")
)]
pub async fn read_file(
    context: &dyn AgentContext,
    file_name: &str,
) -> Result<ToolOutput, ToolError> {
    let cmd = Command::ReadFile(file_name.into());

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
    let cmd = Command::WriteFile(file_name.into(), content.into());

    let output = context.exec_cmd(&cmd).await?;

    match output {
        CommandOutput::Shell { success, .. } if success => {
            return Ok("File written succesfully".into())
        }
        CommandOutput::Shell {
            success, stderr, ..
        } if !success => {
            return Err(anyhow::anyhow!("Failed to write file: {}", stderr).into());
        }
        _ => {
            return Err(anyhow::anyhow!("Unexpected output from write file").into());
        }
    };
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
            CommandOutput::Shell {
                stdout, success, ..
            } if success => stdout,
            CommandOutput::Shell {
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

        // Any errors we just forward to the llm at this point
        let response = self
            .github_session
            .create_pull_request(
                current_branch,
                &self.github_session.main_branch(),
                title,
                pull_request_body,
            )
            .await
            .map_or_else(
                |e| Ok::<String, ToolError>(e.to_string()),
                |pr| Ok(format!("Created a pull request at `{}`", pr.url)),
            )
            .unwrap();

        Ok(response.into())
    }
}

#[derive(Tool, Clone, Debug)]
#[tool(description = "Runs tests")]
pub struct RunTests {
    pub test_command: String,
}

impl RunTests {
    pub fn new(test_command: impl AsRef<str>) -> Self {
        Self {
            test_command: test_command.as_ref().to_string(),
        }
    }

    async fn run_tests(&self, context: &dyn AgentContext) -> Result<ToolOutput, ToolError> {
        let cmd = Command::Shell(self.test_command.clone());
        let output = context.exec_cmd(&cmd).await?;

        Ok(output.into())
    }
}
