use std::sync::Arc;

use crate::{
    chat_message::ChatMessage,
    commands::{Command, CommandEvent, CommandResponse, Responder},
    frontend::{ui_event::UIEvent, App},
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

    // Error handling should probably just forward the error to the UI
    let diff_message = match rx.recv().await.expect("Failed to receive diff") {
        CommandResponse::Activity(_, payload) => payload,
        msg => panic!("Expected chat message got {msg:?}"),
    };

    app.send_ui_event(UIEvent::ChatMessage(
        current_chat_uuid,
        ChatMessage::new_system(diff_message),
    ));
}

pub async fn diff_pull(app: &mut App<'_>) {}
