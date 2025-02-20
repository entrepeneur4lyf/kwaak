use anyhow::{Context as _, Result};
use std::sync::Arc;
use swiftide_macros::{tool, Tool};
use uuid::Uuid;

use super::{
    agent_session::Session,
    conversation_summarizer::ConversationSummarizer,
    env_setup::{self, EnvSetup},
    tool_summarizer::ToolSummarizer,
    tools, RunningAgent,
};
use swiftide::{
    agents::{
        system_prompt::SystemPrompt, tools::local_executor::LocalExecutor, Agent, DefaultContext,
    },
    chat_completion::{self, errors::ToolError, ChatCompletion, Tool, ToolOutput},
    prompt::Prompt,
    traits::{AgentContext, Command, SimplePrompt, ToolExecutor},
};

use crate::{commands::Responder, git::github::GithubSession, indexing, repository::Repository};

use super::env_setup::AgentEnvironment;

#[derive(Clone, Tool)]
#[tool(
    description = "Delegate to the coding agent",
    param(
        name = "task",
        description = "A thorough description of the task to be completed"
    )
)]
pub struct RunCodingAgent {
    session: Arc<Session>,
}

impl RunCodingAgent {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    pub async fn run_coding_agent(
        &self,
        context: &dyn AgentContext,
        task: &str,
    ) -> Result<ToolOutput, ToolError> {
        // Start the agent on this session
        // TODO:
        // - How do we deal with output
        // - Any additional prompting etc
        // - Track the agent; only allow one agent per session
        // - Keep it simple -> Delegating to this agent will make this the active agent
        //
        Ok("".into())
    }
}

pub async fn start(query: &str, session: Session) -> Result<RunningAgent> {
    // Ensure the session is set up
    // tools, etc => Session should provide:
    // - providers
    // - github session
    // - branch_name / executor / initial context
    // - env_setup / agent env
    // - all tools
    // - Tracks running agent?
    // - Interaction with command responder?
    // - uuid = session id
    //
    let executor = Arc::clone(&session.executor());
    let mut context = Arc::new(DefaultContext::from_executor(Arc::clone(&executor)));
    let initial_context = generate_initial_context(&session.repository(), query).await?;

    let query_provider: Box<dyn ChatCompletion> =
        session.repository().config().query_provider().try_into()?;
    let fast_query_provider: Box<dyn SimplePrompt> = session
        .repository()
        .config()
        .indexing_provider()
        .try_into()?;

    let system_prompt = SystemPrompt::builder().build()?;
    let tools = session
        .available_tools()
        .iter()
        .filter_map(|tool| {
            if [
                "search_file",
                "search_code",
                "fetch_url",
                "explain_code",
                "read_file",
                "github_search_code",
                "search_web",
                "run_tests",
                "run_coverage",
            ]
            .contains(&tool.name())
            {
                Some(tool.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let tool_summarizer = ToolSummarizer::new(
        fast_query_provider,
        &["run_tests", "run_coverage"],
        &tools,
        &session.agent_environment().start_ref,
    );
    let conversation_summarizer = ConversationSummarizer::new(
        query_provider.clone(),
        &tools,
        &session.agent_environment().start_ref,
    );
    let responder = session.responder_clone();

    // tmp
    let tx1 = responder.clone();
    let tx2 = responder.clone();
    let tx3 = responder.clone();

    let agent = Agent::builder()
        .context(Arc::clone(&context) as Arc<dyn AgentContext>)
        .system_prompt(system_prompt)
        .tools(tools)
        .before_all(move |context| {
            let initial_context = initial_context.clone();

            Box::pin(async move {
                context
                    .add_message(chat_completion::ChatMessage::new_user(initial_context))
                    .await;

                let top_level_project_overview = context.exec_cmd(&Command::shell("fd -iH -d2 -E '.git/*'")).await?.output;
                context.add_message(chat_completion::ChatMessage::new_user(format!("The following is a max depth 2, high level overview of the directory structure of the project: \n ```{top_level_project_overview}```"))).await;

                Ok(())
            })
        })
        .on_new_message(move |_, message| {
            let command_responder = tx1.clone();
            let message = message.clone();

            Box::pin(async move {
                command_responder.agent_message(message);

                Ok(())
            })
        })
        .before_completion(move |_, _| {
            let command_responder = tx2.clone();
            Box::pin(async move {
                command_responder.update("running completions");
                Ok(())
            })
        })
        .before_tool(move |_, tool| {
            let command_responder = tx3.clone();
            let tool = tool.clone();
            Box::pin(async move {
                command_responder.update(&format!("running tool {}", tool.name()));
                Ok(())
            })
        })
        .after_tool(tool_summarizer.summarize_hook())
        .after_each(conversation_summarizer.summarize_hook())
        .llm(&query_provider)
        .build()?;

    RunningAgent::builder()
        .agent(agent)
        .executor(executor)
        .agent_environment(session.agent_environment().clone())
        .agent_context(context as Arc<dyn AgentContext>)
        .build()
}

async fn generate_initial_context(repository: &Repository, query: &str) -> Result<String> {
    let retrieved_context = indexing::query(repository, &query).await?;
    let formatted_context = format!("Additional information:\n\n{retrieved_context}");
    Ok(formatted_context)
}
