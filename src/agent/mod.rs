mod docker_tool_executor;
mod tools;

use std::{future::IntoFuture, sync::Arc};

use anyhow::{anyhow, Result};
use docker_tool_executor::DockerExecutor;
use swiftide::{
    agents::{Agent, DefaultContext},
    chat_completion::{ChatCompletion, ChatMessage},
};

use crate::{
    query::{self, query},
    repository::Repository,
};

pub async fn run_agent(repository: &Repository, query: &str) -> Result<String> {
    let query_provider: Box<dyn ChatCompletion> =
        repository.config().query_provider().try_into()?;

    let repository = Arc::new(repository.clone());
    let query = query.to_string();
    let executor = DockerExecutor::from_repository(&repository).await?;
    let context = DefaultContext::from_executor(executor);

    let mut agent = Agent::builder()
        .context(context)
        .before_all(move |context| {
            let repository = repository.clone();
            let query = query.clone();

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
