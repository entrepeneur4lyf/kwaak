#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::{config::Config, repository::Repository};
    use uuid::Uuid;
    use tokio::sync::{mpsc, RwLock};
    use crate::frontend::App;

    // Mock configuration for testing
    fn get_mock_config() -> Config {
        toml::from_str(r#"
            language = "rust"

            [commands]
            test = "cargo test"
            coverage = "cargo tarpaulin"

            [github]
            owner = "test-owner"
            repository = "test-repo"
            token = "text:test-token"

            [llm]
            provider = "OpenAI"
            api_key = "text:test-key"
            prompt_model = "gpt-4o-mini"
        "#).unwrap()
    }

    // Test CommandHandler for starting an agent
    #[tokio::test]
    async fn test_start_agent() {
        let config = get_mock_config();
        let repository = Repository::from_config(config);

        // Prepare command handler
        let command_handler = CommandHandler::from_repository(repository.clone());
        let uuid = Uuid::new_v4();

        // Use the command handler to attempt starting an agent
        let result = command_handler.find_or_start_agent_by_uuid(uuid, "test query").await;

        // Check if the agent was successfully retrieved or started
        assert!(result.is_ok(), "Failed to start or find agent");
    }
}
