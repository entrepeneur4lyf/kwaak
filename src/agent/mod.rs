mod docker_tool_executor;
mod env_setup;
mod tools;

use std::sync::Arc;

use anyhow::Result;
use docker_tool_executor::DockerExecutor;
use env_setup::EnvSetup;
use swiftide::{
    agents::{system_prompt::SystemPrompt, Agent, DefaultContext},
    chat_completion::{self, ChatCompletion, Tool},
};
use tokio::sync::mpsc;

use crate::{
    chat_message::ChatMessage, commands::CommandResponse, git::github::GithubSession, indexing,
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
        tools::shell_command(),
        tools::CreatePullRequest::new(&github_session).boxed(),
        tools::RunTests::new(&repository.config().commands.test).boxed(),
    ];

    let tx_1 = command_response_tx.clone();
    let tx_2 = command_response_tx.clone();

    let context_query = indoc::formatdoc! {r#"
        What is the purpose of the {project_name} that is written in {lang}? Provide a detailed answer to help me understand the context.

        Also consider what else might be helpful to accomplish the following:
        `{query}`
        "#,
        project_name = repository.config().project_name,
        lang = repository.config().language
    };
    let system_prompt =
    SystemPrompt::builder()
        .role("You are an ai agent tasked with helping a user with a code project.")
        .constraints(["If you need to create a pull request, ensure you are on a new branch and have committed your changes"]).build()?;

    let agent = Agent::builder()
        .context(context)
        .system_prompt(system_prompt)
        .tools(tools)
        .before_all(move |context| {
            let repository = repository.clone();
            let command_response_tx = tx_1.clone();
            let context_query = context_query.clone();

            Box::pin(async move {
                command_response_tx
                    .send(
                        ChatMessage::new_system("Generating initial context for agent ...")
                            .build()
                            .into(),
                    )
                    .unwrap();

                let retrieved_context = indexing::query(&repository, &context_query).await?;
                let formatted_context = format!("Additional information:\n\n{retrieved_context}");

                context
                    .add_message(&chat_completion::ChatMessage::User(formatted_context))
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
