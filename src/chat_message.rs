use crate::commands::Command;

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

impl std::fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatMessage::User(msg) => write!(f, "You: {}", msg),
            ChatMessage::System(msg) => write!(f, "System: {}", msg),
            ChatMessage::Command(cmd) => write!(f, "Command: {}", cmd),
        }
    }
}
