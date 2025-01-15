use std::sync::Arc;

use crate::{
    chat_message::ChatMessage,
    commands::{Command, CommandEvent, CommandResponse, Responder},
    frontend::{
        app_command_responder::AppCommandResponderForChatId, ui_event::UIEvent, App,
        AppCommandResponder,
    },
};
use swiftide::traits::Command as ExecCmd;

// TODO: Remove panics :))
pub async fn diff_show(app: &mut App<'_>) {
    // Create a oneshot
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let current_chat_uuid = app.current_chat_uuid;

    let event = CommandEvent::builder()
        .command(Command::Diff)
        .uuid(current_chat_uuid)
        .responder(Arc::new(tx))
        .build()
        .expect("Infallible; should not fail to build event for diff show");

    app.command_tx
        .as_ref()
        .expect("Command tx not set")
        .send(event)
        .expect("Failed to dispatch command");

    // App tx so we forward everything else
    // TODO: Think of a nicer way to do this. It's a bit hacky. Maybe a forwarder?
    let app_tx = app.command_responder.for_chat_id(current_chat_uuid);
    let mut diff_message = String::new();
    while let Some(msg) = rx.recv().await {
        match msg {
            CommandResponse::BackendMessage(_, ref payload) => {
                if diff_message.is_empty() {
                    diff_message = payload.to_string();

                    app.send_ui_event(UIEvent::ChatMessage(
                        current_chat_uuid,
                        ChatMessage::new_system(diff_message.clone()),
                    ));
                } else {
                    app_tx.send(msg);
                }
            }
            CommandResponse::Completed(_) => {
                app_tx.send(msg);
                break;
            }
            _ => app_tx.send(msg),
        }
    }

    tracing::debug!("Diff message: {}", diff_message);

    // TODO: Diff should be rendered with i.e. ansi-to-tui here
}

pub async fn diff_pull(app: &mut App<'_>) {}
