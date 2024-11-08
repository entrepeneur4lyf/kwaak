use std::{future::IntoFuture, sync::Arc};

use anyhow::{anyhow, Result};
use swiftide::{
    agents::Agent,
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

    let mut agent = Agent::builder()
        .instructions(query.to_string())
        .before_all(move |context| {
            let repository = repository.clone();
            let query = query.to_string();

            Box::pin(async move {
                let retrieved_context = query::query(&repository, &query).await?;

                context
                    .record_in_history(ChatMessage::User(retrieved_context))
                    .await;

                Ok(())
            })
        })
        .llm(&query_provider)
        .build()?;

    agent.run().await?;

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
