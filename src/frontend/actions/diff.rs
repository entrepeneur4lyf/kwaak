use std::sync::Arc;

use crate::{
    commands::{Command, CommandEvent, CommandResponse},
    frontend::{ui_event::UIEvent, App},
};

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

    let diff_message = match rx.recv().await.expect("Failed to receive diff") {
        CommandResponse::Chat(_, message) => message,
        _ => panic!("Expected chat message"),
    };

    app.send_ui_event(UIEvent::ChatMessage(current_chat_uuid, diff_message.into()));
}

pub async fn diff_pull(app: &mut App<'_>) {}
