use crossterm::event::KeyEvent;
use uuid::Uuid;

use crate::{
    chat_message::{ChatMessage, ChatMessageBuilder},
    commands::Command,
};

// Event handling
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum UIEvent {
    Input(KeyEvent),
    Tick,
    Command(Command),
    ChatMessage(ChatMessage),
}

impl From<ChatMessage> for UIEvent {
    fn from(msg: ChatMessage) -> Self {
        Self::ChatMessage(msg)
    }
}

impl From<ChatMessageBuilder> for UIEvent {
    fn from(mut builder: ChatMessageBuilder) -> Self {
        Self::ChatMessage(builder.build())
    }
}

impl From<&mut ChatMessageBuilder> for UIEvent {
    fn from(builder: &mut ChatMessageBuilder) -> Self {
        Self::ChatMessage(builder.build().to_owned())
    }
}
impl From<Command> for UIEvent {
    fn from(cmd: Command) -> Self {
        Self::Command(cmd)
    }
}

impl From<KeyEvent> for UIEvent {
    fn from(key: KeyEvent) -> Self {
        Self::Input(key)
    }
}
