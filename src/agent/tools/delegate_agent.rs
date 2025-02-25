use std::sync::Arc;

use async_trait::async_trait;
use derive_builder::Builder;
use swiftide::{
    agents::{
        system_prompt::SystemPrompt, tools::local_executor::LocalExecutor, Agent, DefaultContext,
    },
    chat_completion::{
        self, errors::ToolError, ChatCompletion, Tool, ToolOutput, ToolSpec, ToolSpecBuilder,
    },
    prompt::Prompt,
    traits::{AgentContext, Command, SimplePrompt, ToolExecutor},
};
use swiftide_macros::Tool;

use crate::agent::{
    running_agent::RunningAgent,
    session::{RunningSession, Session},
};

/// A generic tool to delegate the current task to a new or running agent
// #[derive(Clone, Tool)]
// #[tool(
//     description = "Delegate a task to a specialized agent",
//     param(
//         name = "task",
//         description = "A thorough description of the task to be completed"
//     )
// )]
#[derive(Clone, Builder)]
pub struct DelegateAgent {
    session: Arc<RunningSession>,
    // TODO: Can it be just an agent?
    agent: RunningAgent,

    tool_spec: ToolSpec,
}

impl DelegateAgent {
    pub fn builder() -> DelegateAgentBuilder {
        DelegateAgentBuilder::default()
    }

    pub async fn delegate_agent(
        &self,
        context: &dyn AgentContext,
        task: &str,
    ) -> Result<ToolOutput, ToolError> {
        // TODO: Prompting, etc
        self.session.swap_agent(self.agent.clone());
        self.agent.query(task).await?;

        tracing::info!("Delegated task to agent");
        Ok(ToolOutput::Stop)
    }
}

#[async_trait]
impl Tool for DelegateAgent {
    async fn invoke(
        &self,
        agent_context: &dyn AgentContext,
        raw_args: Option<&str>,
    ) -> Result<ToolOutput, ToolError> {
        todo!()
    }

    fn tool_spec(&self) -> chat_completion::ToolSpec {
        self.tool_spec.clone()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}
