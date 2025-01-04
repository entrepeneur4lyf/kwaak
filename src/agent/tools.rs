use tavily::Tavily;
use std::sync::Arc;
use crate::config::ApiKey;
use crate::AgentContext;
use swiftide::chat_completion::{ToolOutput, errors::ToolError};

pub struct SearchWeb {
    tavily_client: Arc<Tavily>,
    api_key: ApiKey,
}

impl SearchWeb {
    pub fn new(tavily_client: Tavily, api_key: ApiKey) -> Self {
        Self {
            tavily_client: Arc::new(tavily_client),
            api_key,
        }
    }

    async fn search_web(
        &self,
        _context: &dyn AgentContext,
        query: &str,
    ) -> Result<ToolOutput, ToolError> {
        let response = self
            .tavily_client
            .search(query)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(response.into())
    }
}
