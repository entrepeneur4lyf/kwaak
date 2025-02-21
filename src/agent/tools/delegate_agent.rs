use std::sync::Arc;

use swiftide::{
    agents::{
        system_prompt::SystemPrompt, tools::local_executor::LocalExecutor, Agent, DefaultContext,
    },
    chat_completion::{self, errors::ToolError, ChatCompletion, Tool, ToolOutput},
    prompt::Prompt,
    traits::{AgentContext, Command, SimplePrompt, ToolExecutor},
};
use swiftide_macros::Tool;

use crate::agent::{
    running_agent::RunningAgent,
    session::{RunningSession, Session},
};

/// A generic tool to delegate the current task to a new or running agent
#[derive(Clone, Tool)]
#[tool(
    description = "Delegate a task to a specialized agent",
    param(
        name = "task",
        description = "A thorough description of the task to be completed"
    )
)]
pub struct DelegateAgent {
    session: Arc<RunningSession>,
    agent: RunningAgent,
}

impl DelegateAgent {
    pub fn new(session: Arc<RunningSession>, agent: RunningAgent) -> Self {
        Self { session, agent }
    }

    pub async fn delegate_agent(
        &self,
        context: &dyn AgentContext,
        task: &str,
    ) -> Result<ToolOutput, ToolError> {
        // TODO: Prompting, etc
        self.session.swap_agent(self.agent.clone());
        self.agent.query(task).await?;

        // TODO: How do we swap back?

        Ok("Agent has completed its task".into())
    }
}
