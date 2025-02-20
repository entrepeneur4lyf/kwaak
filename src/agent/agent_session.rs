use std::sync::Arc;

use anyhow::{Context as _, Result};
use swiftide::{
    agents::{
        system_prompt::SystemPrompt, tools::local_executor::LocalExecutor, Agent, DefaultContext,
    },
    chat_completion::{self, errors::ToolError, ChatCompletion, Tool, ToolOutput},
    prompt::Prompt,
    traits::{AgentContext, Command, SimplePrompt, ToolExecutor},
};
use swiftide_macros::Tool;
use uuid::Uuid;

use crate::{commands::Responder, git::github::GithubSession, repository::Repository};

use super::{env_setup::AgentEnvironment, RunningAgent};

pub struct Session {
    session_id: Uuid,
    repository: Repository,
    agent_environment: Option<AgentEnvironment>,
    responder: Arc<dyn Responder>,

    // After calling init
    github_session: Option<GithubSession>,
    executor: Option<Arc<dyn ToolExecutor>>,
    available_tools: Option<Vec<Box<dyn Tool>>>,
    // The agent that is currently running
    // active_agent: RunningAgent,
}

impl Session {
    pub fn new(session_id: Uuid, repository: Repository, responder: Arc<dyn Responder>) -> Self {
        Self {
            session_id,
            repository,
            responder,
            agent_environment: None,
            github_session: None,
            executor: None,
            available_tools: None,
        }
    }

    pub async fn init(&self) -> Result<()> {
        Ok(())
    }

    pub fn repository(&self) -> &Repository {
        &self.repository
    }

    pub fn github_session(&self) -> Option<&GithubSession> {
        self.github_session.as_ref()
    }

    pub fn executor(&self) -> Arc<dyn ToolExecutor> {
        Arc::clone(
            self.executor
                .as_ref()
                .expect("Agent session not initialized"),
        )
    }

    pub fn agent_environment(&self) -> &AgentEnvironment {
        self.agent_environment
            .as_ref()
            .expect("Agent session not initialized")
    }

    pub fn available_tools(&self) -> &[Box<dyn Tool>] {
        self.available_tools
            .as_ref()
            .expect("Agent session not initialized")
            .as_slice()
    }

    pub fn responder(&self) -> &dyn Responder {
        self.responder.as_ref()
    }

    pub fn responder_clone(&self) -> Arc<dyn Responder> {
        Arc::clone(&self.responder)
    }
}
