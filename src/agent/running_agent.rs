use anyhow::Result;
use derive_builder::Builder;
use std::sync::Arc;
use swiftide::traits::{AgentContext, ToolExecutor};

use swiftide::agents::Agent;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use super::env_setup::AgentEnvironment;

/// Defines any agent that is running
#[derive(Clone, Builder)]
#[builder(build_fn(error = anyhow::Error))]
pub struct RunningAgent {
    /// The agent that is running
    #[builder(setter(custom))]
    pub agent: Arc<Mutex<Agent>>,
    /// The content the agent is running with
    #[builder(setter(into))]
    pub agent_context: Arc<dyn AgentContext>,
}

impl RunningAgent {
    #[must_use]
    pub fn builder() -> RunningAgentBuilder {
        RunningAgentBuilder::default()
    }

    pub async fn query(&self, query: &str) -> Result<()> {
        self.agent.lock().await.query(query).await
    }

    pub async fn run(&self) -> Result<()> {
        self.agent.lock().await.run().await
    }

    pub async fn stop(&self) {
        self.agent.lock().await.stop();
    }
}

impl RunningAgentBuilder {
    pub fn agent(&mut self, agent: Agent) -> &mut Self {
        self.agent = Some(Arc::new(Mutex::new(agent)));
        self
    }
}
