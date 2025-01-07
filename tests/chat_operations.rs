use kwaak::chat::Chat;
use kwaak::commands::{Command, CommandHandler};
use kwaak::frontend::App;
use tempfile::TempDir;
use uuid::Uuid;

struct ChatEnvironment {
    app: App<'static>,
    command_handler: CommandHandler,
    dir: TempDir,
    uuids: Vec<Uuid>,
}

fn setup_chat_env() -> ChatEnvironment {
    let dir = tempfile::Builder::new()
        .prefix("kwaak-chat-test")
        .tempdir()
        .unwrap();

    // Initialize the app and command handler
    let mut app = App::default();
    let mut command_handler = CommandHandler::from_repository("");

    // Register the app with the command handler
    command_handler.register_ui(&mut app);

    ChatEnvironment {
        app,
        command_handler,
        dir,
        uuids: Vec::new(),
    }
}

#[test_log::test(tokio::test)]
async fn test_delete_single_chat() {
    let mut env = setup_chat_env();

    // Simulate adding a chat and storing its UUID
    let chat_uuid = Uuid::new_v4();
    env.uuids.push(chat_uuid);
    env.app.add_chat(Chat::default());

    // Dispatch command to delete the chat
    env.command_handler
        .handle_command(
            &env.command_handler.repository,
            &Command::DeleteChat { uuid: chat_uuid },
        )
        .await
        .expect("Command should succeed");

    // Verify chat is not present in the list of chats
    let chat_present = env.app.chats.iter().any(|chat| chat.uuid == chat_uuid);
    assert!(!chat_present, "Chat should be deleted");
}

#[test_log::test(tokio::test)]
async fn test_delete_all_chats_and_verify_default() {
    let mut env = setup_chat_env();

    // Simulate adding chats
    for _ in 0..3 {
        let chat_uuid = Uuid::new_v4();
        env.uuids.push(chat_uuid);
        env.app.add_chat(Chat::default());
    }

    // Simulate deleting all chats
    for chat_uuid in &env.uuids {
        env.command_handler
            .handle_command(
                &env.command_handler.repository,
                &Command::DeleteChat { uuid: *chat_uuid },
            )
            .await
            .expect("Command should succeed");
    }

    // Check if a new default chat is created
    let default_chat_present = env.app.chats.iter().any(|chat| chat.name == "Chat #1");
    assert!(default_chat_present, "A default chat should exist");
}
