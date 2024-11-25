mod docker_tool_executor;
mod env_setup;
mod tools;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use docker_tool_executor::DockerExecutor;
use env_setup::EnvSetup;
use swiftide::{
    agents::{Agent, DefaultContext},
    chat_completion::{self, ChatCompletion, Tool},
};
use tokio::sync::mpsc;

use crate::{
    chat_message::ChatMessage,
    commands::CommandResponse,
    frontend::UIEvent,
    git::github::GithubSession,
    query::{self},
    repository::Repository,
};

pub async fn build_agent(
    repository: &Repository,
    query: &str,
    command_response_tx: mpsc::UnboundedSender<CommandResponse>,
) -> Result<Agent> {
    command_response_tx
        .send(
            ChatMessage::new_system(
                "Starting up agent for the first time, this might take a while ...",
            )
            .build()
            .into(),
        )
        .unwrap();

    let query_provider: Box<dyn ChatCompletion> =
        repository.config().query_provider().try_into()?;

    let repository = Arc::new(repository.clone());
    let query_for_agent = query.to_string();

    let executor = DockerExecutor::from_repository(&repository).start().await?;
    let github_session = Arc::new(GithubSession::from_repository(&repository)?);

    // Run a series of commands inside the executor so that everything is available
    let env_setup = EnvSetup::new(&repository, &github_session, &executor);
    env_setup.exec_setup_commands().await?;

    let context = DefaultContext::from_executor(executor);

    let tools = vec![
        tools::read_file(),
        tools::write_file(),
        tools::search_file(),
        tools::git(),
        tools::CreatePullRequest::new(&github_session).boxed(),
    ];

    let tx_1 = command_response_tx.clone();
    let tx_2 = command_response_tx.clone();

    let agent = Agent::builder()
        .context(context)
        .tools(tools)
        .before_all(move |context| {
            let repository = repository.clone();
            let query = query_for_agent.clone();
            let command_response_tx = tx_1.clone();

            Box::pin(async move {
                let retrieved_context = query::query(&repository, &query).await?;
                command_response_tx
                    .send(
                        ChatMessage::new_system("Generating dumb context for agent ...")
                            .build()
                            .into(),
                    )
                    .unwrap();

                context
                    .add_message(&chat_completion::ChatMessage::User(retrieved_context))
                    .await;

                Ok(())
            })
        })
        .on_new_message(move |_, message| {
            let command_response_tx = tx_2.clone();
            let message = message.clone();

            Box::pin(async move {
                command_response_tx
                    .send(CommandResponse::Chat(message.into()))
                    .unwrap();

                Ok(())
            })
        })
        .llm(&query_provider)
        .build()?;

    Ok(agent)
}
