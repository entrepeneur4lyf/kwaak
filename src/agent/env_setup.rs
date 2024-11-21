//! Series of commands to run before each agent starts inside the docker container
//!
//!
//!

use anyhow::bail;
use anyhow::Result;
use secrecy::ExposeSecret;
use swiftide::traits::Command;
use swiftide::traits::Output;
use swiftide::traits::ToolExecutor as _;

use crate::{git::github, repository::Repository};

use super::docker_tool_executor::DockerExecutor;
use super::docker_tool_executor::RunningDockerExecutor;

pub struct EnvSetup<'a> {
    repository: &'a Repository,
    executor: &'a RunningDockerExecutor,
}

impl EnvSetup<'_> {
    pub fn new<'a>(
        repository: &'a Repository,
        executor: &'a RunningDockerExecutor,
    ) -> EnvSetup<'a> {
        EnvSetup {
            repository,
            executor,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn exec_setup_commands(&self) -> Result<()> {
        let Output::Shell { stdout, .. } = self
            .executor
            .exec_cmd(&Command::shell("git remote get-url origin"))
            .await?
        else {
            bail!("Could not get origin url")
        };

        let url_with_token =
            github::add_token_to_url(&stdout, &self.repository.config().github_token)?;

        self.executor
            .exec_cmd(&Command::shell(format!(
                "git remote set-url origin {}",
                url_with_token.expose_secret()
            )))
            .await?;

        Ok(())
    }
}
