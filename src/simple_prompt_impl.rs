use crate::config::Config;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use swiftide::prompt::Prompt;
use swiftide::traits::SimplePrompt;

// Define a concrete type that implements SimplePrompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcretePromptProvider;

#[async_trait]
impl SimplePrompt for ConcretePromptProvider {
    async fn prompt(&self, mut prompt: Prompt) -> Result<String> {
        // Use with_context_value() if needed for adding specific context
        // Render the prompt to process and provide a response
        prompt = prompt.with_context_value("key", "value"); // Example context
        let rendered_result = prompt.render().await?;
        Ok(format!("Response to: {rendered_result}"))
    }
}

// Implement TryFrom<Config> for Box<dyn SimplePrompt>
impl TryFrom<&Config> for Box<dyn SimplePrompt> {
    type Error = anyhow::Error;

    fn try_from(_config: &Config) -> Result<Self, Self::Error> {
        // Here you configure the ConcretePromptProvider based on passed config if needed
        Ok(Box::new(ConcretePromptProvider))
    }
}
