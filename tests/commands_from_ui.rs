use kwaak::chat::Chat;
use kwaak::commands::{Command, CommandHandler};
use kwaak::frontend::{ui, App, AppCommandResponder, DiffVariant, UIEvent, UserInputCommand};
use kwaak::{storage, test_utils};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use swiftide_core::Persist;
use uuid::Uuid;

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_diff() {
    let mut app = App::default();
    let (repository, _guard) = test_utils::test_repository();
    let lancedb = storage::get_lancedb(&repository);
    lancedb.setup().await.unwrap();

    let mut handler = CommandHandler::from_repository(repository);
    handler.register_ui(&mut app);
    let _handler_guard = handler.start();

    let fixed_uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    let Some(current_chat) = app.current_chat_mut() else {
        panic!("No current chat");
    };

    // Force to fixed uuid so that snapshots are stable
    current_chat.uuid = fixed_uuid;
    app.current_chat_uuid = fixed_uuid;

    // First, let's start a noop agent so an environment is running
    app.dispatch_command(
        fixed_uuid,
        Command::Chat {
            message: "hello".to_string(),
        },
    );

    let event = app
        .handle_events_until(UIEvent::is_command_done)
        .await
        .unwrap();

    assert_eq!(event, UIEvent::CommandDone(fixed_uuid));

    // The user asks for a diff, it should be empty
    app.send_ui_event(UIEvent::UserInputCommand(
        fixed_uuid,
        UserInputCommand::Diff(DiffVariant::Show),
    ));

    let event = app
        .handle_events_until(UIEvent::is_command_done)
        .await
        .unwrap();

    assert_eq!(event, UIEvent::CommandDone(fixed_uuid));

    // Render the main chat screen with all the state changes
    let mut terminal = Terminal::new(TestBackend::new(160, 40)).unwrap();
    terminal.draw(|f| ui(f, f.area(), &mut app)).unwrap();
    insta::assert_snapshot!(terminal.backend());

    // otherwise the tests will hang -.-
    // handler_guard.abort();
}
