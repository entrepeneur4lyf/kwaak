use std::sync::Arc;

use anyhow::Result;
use swiftide::{
    agents::{system_prompt::SystemPrompt, Agent, DefaultContext},
    chat_completion::{self, ChatCompletion, Tool},
};
use tavily::Tavily;

use crate::{
    commands::CommandResponder, git::github::GithubSession, indexing, repository::Repository,
};

use super::{docker_tool_executor::DockerExecutor, env_setup::EnvSetup, tools};

#[tracing::instrument(skip(repository, command_responder))]
pub async fn build_agent(
    repository: &Repository,
    query: &str,
    command_responder: CommandResponder,
) -> Result<Agent> {
    command_responder.send_update("starting up agent for the first time, this might take a while");

    let query_provider: Box<dyn ChatCompletion> =
        repository.config().query_provider().try_into()?;

    let repository = Arc::new(repository.clone());

    let executor = DockerExecutor::from_repository(&repository).start().await?;
    let github_session = Arc::new(GithubSession::from_repository(&repository)?);

    // Run a series of commands inside the executor so that everything is available
    let env_setup = EnvSetup::new(&repository, &github_session, &executor);
    env_setup.exec_setup_commands().await?;

    let context = DefaultContext::from_executor(executor);

    let query_pipeline = indexing::build_query_pipeline(&repository)?;
    let mut tools = vec![
        tools::read_file(),
        tools::write_file(),
        tools::search_file(),
        tools::git(),
        tools::shell_command(),
        tools::SearchCode::new(query_pipeline).boxed(),
        tools::CreatePullRequest::new(&github_session).boxed(),
        tools::RunTests::new(&repository.config().commands.test).boxed(),
    ];

    if let Some(tavily_api_key) = &repository.config().tavily_api_key {
        // Client is a bit weird that it needs the api key twice
        // Maybe roll our own? It's just a rest api
        let tavily = Tavily::new(tavily_api_key.expose_secret());
        tools.push(tools::SearchWeb::new(tavily, tavily_api_key.clone()).boxed());
    }

    let command_responder = Arc::new(command_responder);
    // Maybe I'm just too tired but feels off.
    let tx_1 = command_responder.clone();
    let tx_2 = command_responder.clone();
    let tx_3 = command_responder.clone();
    let tx_4 = command_responder.clone();

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
        .constraints([
            "If you need to create a pull request, ensure you are on a new branch and have committed your changes",
            "Research your solution before providing it",
            "When writing files, ensure you write and implement everything, everytime. Do NOT leave anything out",
            "Tool calls are in parallel. You can run multiple tool calls at the same time, but they must not rely on eachother"
        ]).build()?;

    let agent = Agent::builder()
        .context(context)
        .system_prompt(system_prompt)
        .tools(tools)
        .before_all(move |context| {
            let repository = repository.clone();
            let command_responder = tx_1.clone();
            let context_query = context_query.clone();

            Box::pin(async move {
                command_responder.send_update("generating initial context");

                let retrieved_context = indexing::query(&repository, &context_query).await?;
                let formatted_context = format!("Additional information:\n\n{retrieved_context}");

                context
                    .add_message(&chat_completion::ChatMessage::User(formatted_context))
                    .await;

                Ok(())
            })
        })
        .on_new_message(move |_, message| {
            let command_responder = tx_2.clone();
            let message = message.clone();

            Box::pin(async move {
                command_responder.send_message(message);

                Ok(())
            })
        })
        // before each, update that we're running completions
        .before_each(move |_| {
            let command_responder = tx_3.clone();
            Box::pin(async move {
                command_responder.send_update("running completions");
                Ok(())
            })
        })
        .before_tool(move |_, tool| {
            let command_responder = tx_4.clone();
            let tool = tool.clone();
            Box::pin(async move {
                command_responder.send_update(format!("running tool {}", tool.name()));
                Ok(())
            })
        })
        .llm(&query_provider)
        .build()?;

    Ok(agent)
}
