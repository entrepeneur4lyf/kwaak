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
    let handler_guard = handler.start();

    let fixed_uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    let Some(current_chat) = app.current_chat_mut() else {
        panic!("No current chat");
    };

    current_chat.uuid = fixed_uuid;
    app.current_chat_uuid = fixed_uuid;

    AppCommandResponder::init(app.ui_tx.clone()).unwrap();

    // First, let's start a noop agent so an environment is running
    app.dispatch_command(
        fixed_uuid,
        Command::Chat {
            message: "hello".to_string(),
        },
    );

    tracing::debug!("Waiting for messages from starting agent");
    let mut num_messages = 0;

    while let Some(msg) = app.recv_messages().await {
        tracing::info!("[TEST] {}/10 received message: {msg:?}", num_messages);
        num_messages += 1;
        if let UIEvent::CommandDone(uuid) = msg {
            tracing::warn!("[TEST] Received command done: {uuid}");
            assert_eq!(uuid, fixed_uuid);
            break;
        }
        if num_messages > 10 {
            break;
        }
    }

    // while let Some(msg) = app.recv_messages().await {
    //     tracing::info!("Test received message: {msg:?}");
    //     num_messages += 1;
    //     if let UIEvent::CommandDone(uuid) = msg {
    //         assert_eq!(uuid, fixed_uuid);
    //         break;
    //     }
    //     if num_messages > 10 {
    //         break;
    //     }
    // }

    app.send_ui_event(UIEvent::UserInputCommand(
        fixed_uuid,
        UserInputCommand::Diff(DiffVariant::Show),
    ));

    // Ensure the ui event is processed
    app.handle_single_event().await;
    // Ensure that the command is send
    app.handle_single_event().await;

    // And now wait for the diff command to be returned
    tracing::debug!("Waiting for messages from diff");
    let mut num_messages = 0;
    while let Some(msg) = app.recv_messages().await {
        num_messages += 1;
        if let UIEvent::CommandDone(uuid) = msg {
            assert_eq!(uuid, fixed_uuid);
            break;
        }
        tracing::debug!(received = ?msg, "Received message");
        if num_messages > 10 {
            break;
        }
    }
    //
    // let mut terminal = Terminal::new(TestBackend::new(160, 40)).unwrap();
    // terminal.draw(|f| ui(f, f.area(), &mut app)).unwrap();
    // insta::assert_snapshot!(terminal.backend());
    //
    // app.handle_single_event().await;
    //
    // // Converts it to a UIEvent::DiffPull
    // app.handle_single_event().await;
    //
    // // The diff pull command should be sent
    // app.handle_single_event().await;

    // otherwise the tests will hang -.-
    handler_guard.abort();
}
