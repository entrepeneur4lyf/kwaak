use std::sync::{Arc, Mutex};

use anyhow::{Context as _, Result};
use derive_builder::Builder;
use swiftide::{
    agents::tools::local_executor::LocalExecutor,
    chat_completion::Tool,
    traits::{SimplePrompt, ToolExecutor},
};
use swiftide_docker_executor::DockerExecutor;
use tavily::Tavily;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    agent::util,
    commands::Responder,
    config::{AgentEditMode, SupportedToolExecutors},
    git::github::GithubSession,
    indexing,
    repository::Repository,
};

use super::{
    env_setup::{self, AgentEnvironment, EnvSetup},
    running_agent::RunningAgent,
    tools, v1,
};

/// Session represents the abstract state of an ongoing agent interaction (i.e. in a chat)
///
/// TODO: The command responder will instead receive generic session updates
#[derive(Clone, Builder)]
#[builder(build_fn(private), setter(into))]
pub struct Session {
    pub session_id: Uuid,
    pub repository: Arc<Repository>,
    pub default_responder: Arc<dyn Responder>,
    pub initial_query: String,
    // available_tools: Vec<Box<dyn Tool>>,
    //
    // branch name
    // chat name
    // After calling init
    // The agent that is currently running
    // active_agent: RunningAgent,
}

impl Session {
    #[must_use]
    pub fn builder() -> SessionBuilder {
        SessionBuilder::default()
    }
}

impl SessionBuilder {
    pub async fn start(&mut self) -> Result<RunningSession> {
        let session = self.build().context("Failed to build session")?;

        let github_session = match session.repository.config().github_api_key {
            Some(_) => Some(Arc::new(GithubSession::from_repository(
                &session.repository,
            )?)),
            None => None,
        };

        let backoff = session.repository.config().backoff;
        let fast_query_provider: Box<dyn SimplePrompt> = session
            .repository
            .config()
            .indexing_provider()
            .get_simple_prompt_model(backoff)?;

        let ((), executor, branch_name, initial_context) = tokio::try_join!(
            util::rename_chat(
                &session.initial_query,
                &fast_query_provider,
                &session.default_responder
            ),
            start_tool_executor(session.session_id, &session.repository),
            // TODO: Below should probably be agent specific
            util::create_branch_name(
                &session.initial_query,
                &session.session_id,
                &fast_query_provider,
                &session.default_responder
            ),
            generate_initial_context(&session.repository, &session.initial_query)
        )?;

        let env_setup = EnvSetup::new(&session.repository, github_session.as_deref(), &*executor);
        let agent_environment = env_setup.exec_setup_commands(branch_name).await?;

        let available_tools = available_tools(
            &session.repository,
            github_session.as_ref(),
            Some(&agent_environment),
        )?;

        let active_agent = match session.repository.config().agent {
            crate::config::SupportedAgents::V1 => {
                v1::start(
                    &session,
                    &executor,
                    &available_tools,
                    &agent_environment,
                    initial_context,
                )
                .await
            }
        }?;

        Ok(RunningSession {
            active_agent: Arc::new(Mutex::new(active_agent)),
            session: session.into(),
            github_session,
            executor,
            agent_environment,
            available_tools: available_tools.into(),
            cancel_token: Arc::new(Mutex::new(CancellationToken::new())),
        })
    }
}

// TODO: A session could have multiple agents, with one (or more!) active
// Also, maybe full inner mutability?

/// References a running session
/// Meant to be cloned
// TODO: Merge with session?
#[derive(Clone)]
#[allow(dead_code)]
pub struct RunningSession {
    session: Arc<Session>,
    active_agent: Arc<Mutex<RunningAgent>>,

    github_session: Option<Arc<GithubSession>>,
    executor: Arc<dyn ToolExecutor>,
    agent_environment: AgentEnvironment,
    available_tools: Arc<Vec<Box<dyn Tool>>>,

    cancel_token: Arc<Mutex<CancellationToken>>,
}

impl RunningSession {
    /// Get a cheap copy of the active agent
    #[must_use]
    pub fn active_agent(&self) -> RunningAgent {
        self.active_agent.lock().unwrap().clone()
    }

    /// Run an agent with a query
    pub async fn query_agent(&self, query: &str) -> Result<()> {
        self.active_agent().query(query).await
    }

    /// Run an agent without a query
    pub async fn run_agent(&self) -> Result<()> {
        self.active_agent().run().await
    }

    pub fn swap_agent(&self, running_agent: RunningAgent) {
        let mut lock = self.active_agent.lock().unwrap();
        *lock = running_agent;
    }

    #[must_use]
    pub fn executor(&self) -> &dyn ToolExecutor {
        &self.executor
    }

    #[must_use]
    pub fn agent_environment(&self) -> &AgentEnvironment {
        &self.agent_environment
    }

    #[must_use]
    pub fn cancel_token(&self) -> CancellationToken {
        self.cancel_token.lock().unwrap().clone()
    }

    pub fn reset_cancel_token(&self) {
        let mut lock = self.cancel_token.lock().unwrap();
        *lock = CancellationToken::new();
    }

    pub async fn stop(&self) {
        // When sessions have multiple agents, they should be stopped here
        self.reset_cancel_token();
        let lock = self.active_agent.lock().unwrap().clone();
        lock.stop().await;
    }
}

async fn start_tool_executor(uuid: Uuid, repository: &Repository) -> Result<Arc<dyn ToolExecutor>> {
    let boxed = match repository.config().tool_executor {
        SupportedToolExecutors::Docker => {
            let mut executor = DockerExecutor::default();
            let dockerfile = &repository.config().docker.dockerfile;

            if std::fs::metadata(dockerfile).is_err() {
                tracing::error!("Dockerfile not found at {}", dockerfile.display());
                panic!("Running in docker requires a Dockerfile");
            }
            let running_executor = executor
                .with_context_path(&repository.config().docker.context)
                .with_image_name(repository.config().project_name.to_lowercase())
                .with_dockerfile(dockerfile)
                .with_container_uuid(uuid)
                .to_owned()
                .start()
                .await?;

            Arc::new(running_executor) as Arc<dyn ToolExecutor>
        }
        SupportedToolExecutors::Local => Arc::new(LocalExecutor::new(".")) as Arc<dyn ToolExecutor>,
    };

    Ok(boxed)
}

async fn generate_initial_context(repository: &Repository, query: &str) -> Result<String> {
    let retrieved_context = indexing::query(repository, &query).await?;
    let formatted_context = format!("Additional information:\n\n{retrieved_context}");
    Ok(formatted_context)
}

pub fn available_tools(
    repository: &Repository,
    github_session: Option<&Arc<GithubSession>>,
    agent_env: Option<&env_setup::AgentEnvironment>,
) -> Result<Vec<Box<dyn Tool>>> {
    let query_pipeline = indexing::build_query_pipeline(repository)?;

    let mut tools = vec![
        tools::write_file(),
        tools::search_file(),
        tools::git(),
        tools::shell_command(),
        tools::search_code(),
        tools::fetch_url(),
        tools::ExplainCode::new(query_pipeline).boxed(),
    ];

    match repository.config().agent_edit_mode {
        AgentEditMode::Whole => {
            tools.push(tools::write_file());
            tools.push(tools::read_file());
        }
        AgentEditMode::Line => {
            tools.push(tools::read_file_with_line_numbers());
            tools.push(tools::replace_lines());
            tools.push(tools::add_lines());
        }
    }

    if let Some(github_session) = github_session {
        if !repository.config().disabled_tools.pull_request {
            tools.push(tools::CreateOrUpdatePullRequest::new(github_session).boxed());
        }
        tools.push(tools::GithubSearchCode::new(github_session).boxed());
    }

    if let Some(tavily_api_key) = &repository.config().tavily_api_key {
        let tavily = Tavily::builder(tavily_api_key.expose_secret()).build()?;
        tools.push(tools::SearchWeb::new(tavily, tavily_api_key.clone()).boxed());
    };

    if let Some(test_command) = &repository.config().commands.test {
        tools.push(tools::RunTests::new(test_command).boxed());
    }

    if let Some(coverage_command) = &repository.config().commands.coverage {
        tools.push(tools::RunCoverage::new(coverage_command).boxed());
    }

    if let Some(env) = agent_env {
        tools.push(tools::ResetFile::new(&env.start_ref).boxed());
    }

    Ok(tools)
}
