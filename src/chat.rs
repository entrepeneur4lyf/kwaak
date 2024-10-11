use crate::chat_message::ChatMessage;

#[derive(Debug, Clone)]
pub struct Chat {
    /// Display name of the chat
    pub name: String,
    /// Identifier used to match responses
    pub uuid: uuid::Uuid,
    pub messages: Vec<ChatMessage>,
    state: ChatState,
}

impl Chat {
    pub(crate) fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum ChatState {
    Loading,
    NewMessage,
    #[default]
    Ready,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            name: "Chat".to_string(),
            uuid: uuid::Uuid::new_v4(),
            messages: Vec::new(),
            state: ChatState::default(),
        }
    }
}
