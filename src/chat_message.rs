use crate::{app::UIEvent, commands::Command};

/// Represents a chat message that can be stored in the app
pub enum ChatMessage {
    User(String),
    System(String),
    Command(Command),
}

impl ChatMessage {
    pub fn new_user(msg: impl Into<String>) -> ChatMessage {
        ChatMessage::User(msg.into())
    }

    pub fn new_system(msg: impl Into<String>) -> ChatMessage {
        ChatMessage::System(msg.into())
    }

    pub fn new_command(cmd: impl Into<Command>) -> ChatMessage {
        ChatMessage::Command(cmd.into())
    }
}

impl From<ChatMessage> for UIEvent {
    fn from(val: ChatMessage) -> Self {
        UIEvent::ChatMessage(val)
    }
}
