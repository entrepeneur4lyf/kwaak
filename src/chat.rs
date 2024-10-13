use crate::chat_message::ChatMessage;

#[derive(Debug, Clone)]
pub struct Chat {
    /// Display name of the chat
    pub name: String,
    /// Identifier used to match responses
    pub uuid: uuid::Uuid,
    pub messages: Vec<ChatMessage>,
    pub state: ChatState,
}

impl Chat {
    pub(crate) fn add_message(&mut self, message: ChatMessage) {
        if message.role().is_system() {
            self.state = ChatState::NewMessage;
        }
        self.messages.push(message);
    }

    pub fn set_loading(&mut self) {
        self.state = ChatState::Loading;
    }

    pub fn set_ready(&mut self) {
        self.state = ChatState::Ready;
    }

    pub(crate) fn has_new_messages(&self) -> bool {
        self.state.is_new_message()
    }

    pub(crate) fn is_loading(&self) -> bool {
        self.state.is_loading()
    }
}

#[derive(Debug, Clone, Copy, Default, strum::EnumIs)]
pub enum ChatState {
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
