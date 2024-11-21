#![allow(dead_code)]
use anyhow::Result;
use swiftide::{
    chat_completion::ToolOutput,
    traits::{AgentContext, Command},
};
use swiftide_macros::{tool, Tool};

#[tool(
    description = "Reads file content",
    param(name = "file_name", description = "Full path of the file")
)]
pub async fn read_file(context: &dyn AgentContext, file_name: &str) -> Result<ToolOutput> {
    let cmd = Command::Shell(format!("cat {file_name}"));

    context.exec_cmd(&cmd).await.map(Into::into)
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
) -> Result<ToolOutput> {
    let heredoc = format!("<<HERE\n{content}\nHERE");
    let cmd = Command::Shell(format!("echo {heredoc} > {file_name}"));

    context.exec_cmd(&cmd).await.map(Into::into)
}

#[tool(
    description = "Searches for a file",
    param(name = "file_name", description = "Partial or full name of the file")
)]
pub async fn search_file(context: &dyn AgentContext, file_name: &str) -> Result<ToolOutput> {
    let cmd = Command::Shell(format!("find . -name '*{file_name}*'"));
    context.exec_cmd(&cmd).await.map(Into::into)
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

    async fn run_tests(&self, context: &dyn AgentContext) -> Result<ToolOutput> {
        let cmd = Command::Shell(self.test_command.clone());
        context.exec_cmd(&cmd).await.map(Into::into)
    }
}
// read file
// write file
// search file
// run tests
