mod docker_tool_executor;
mod env_setup;
mod tools;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use docker_tool_executor::DockerExecutor;
use env_setup::EnvSetup;
use swiftide::{
    agents::{Agent, DefaultContext},
    chat_completion::{ChatCompletion, ChatMessage},
};

use crate::{
    git::github::GithubSession,
    query::{self},
    repository::Repository,
};

pub async fn run_agent(repository: &Repository, query: &str) -> Result<String> {
    let query_provider: Box<dyn ChatCompletion> =
        repository.config().query_provider().try_into()?;

    let repository = Arc::new(repository.clone());
    let query_for_agent = query.to_string();

    let executor = DockerExecutor::from_repository(&repository).start().await?;
    let github_session = GithubSession::from_repository(&repository)?;

    // Run a series of commands inside the executor so that everything is available
    let env_setup = EnvSetup::new(&repository, &github_session, &executor);
    env_setup.exec_setup_commands().await?;

    let context = DefaultContext::from_executor(executor);

    let mut agent = Agent::builder()
        .context(context)
        .before_all(move |context| {
            let repository = repository.clone();
            let query = query_for_agent.clone();

            Box::pin(async move {
                let retrieved_context = query::query(&repository, &query).await?;

                context
                    .add_message(&ChatMessage::User(retrieved_context))
                    .await;

                Ok(())
            })
        })
        .llm(&query_provider)
        .build()?;

    agent.query(query).await?;

    let response = agent
        .history()
        .await
        .iter()
        .filter_map(|msg| match msg {
            ChatMessage::Assistant(msg) => Some(msg),
            _ => None,
        })
        .last()
        .ok_or(anyhow!("No message found"))
        .cloned();

    response
}
