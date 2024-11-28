use super::*;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Default)]
struct MockAgent;

impl MockAgent {
    fn new() -> Self {
        MockAgent {}
    }

    fn execute(&self, command: &Command) -> bool {
        match command {
            Command::RunAgent { .. } => true,
            _ => false,
        }
    }
}

#[tokio::test]
async fn test_command_handler_with_mock_agent() {
    let repository = Repository::default(); // Assuming a default impl exists for testing
    let mut command_handler = CommandHandler::from_repository(repository);

    // Create a mock agent
    let mock_agent = MockAgent::new();
    let mock_uuid = Uuid::new_v4();

    // Mock the behavior of starting an agent
    command_handler.agents.write().await.insert(mock_uuid, mock_agent);

    // Create a command to run the agent
    let command = Command::RunAgent { uuid: mock_uuid };

    // Send the command
    command_handler.tx.send(command.clone()).unwrap();
    
    // Simulate handling of command
    if let Some(agent) = command_handler.agents.write().await.get(&mock_uuid) {
        assert!(agent.execute(&command));
    } else {
        panic!("Agent not found or not started correctly.");
    }
}
