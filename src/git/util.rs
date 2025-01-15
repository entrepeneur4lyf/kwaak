use anyhow::Result;
use swiftide_core::{Command, ToolExecutor};

use crate::util::accept_non_zero_exit;

pub async fn diff(executor: &dyn ToolExecutor, base_sha: &str, head_sha: &str) -> Result<String> {
    let cmd = Command::shell(format!("git diff {base_sha} {head_sha}",));

    let mut output = accept_non_zero_exit(executor.exec_cmd(&cmd).await)?.output;

    if output.is_empty() {
        output = "No changes".to_string();
    }

    Ok(output)
}
