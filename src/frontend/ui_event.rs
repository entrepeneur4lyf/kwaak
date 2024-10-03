use crossterm::event::KeyEvent;

use crate::{chat_message::ChatMessage, commands::Command};

// Event handling
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
