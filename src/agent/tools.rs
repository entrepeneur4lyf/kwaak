use anyhow::Result;
use swiftide::{
    chat_completion::ToolOutput,
    traits::{AgentContext, Command},
};
use swiftide_macros::tool;

pub async fn read_file(context: &dyn AgentContext, file_name: &str) -> Result<ToolOutput> {
    let cmd = Command::Shell(format!("cat {}", file_name));

    context.exec_cmd(&cmd).await.map(Into::into)
}

pub async fn write_file(
    context: &dyn AgentContext,
    file_name: &str,
    content: &str,
) -> Result<ToolOutput> {
    let cmd = Command::Shell(format!("echo {} > {}", content, file_name));

    context.exec_cmd(&cmd).await.map(Into::into)
}
// read file
// write file
// search file
// run tests
