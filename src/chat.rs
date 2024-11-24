use crate::chat_message::ChatMessage;

#[derive(Debug, Clone)]
pub struct Chat {
    /// Display name of the chat
    pub name: String,
    /// Identifier used to match responses
    pub uuid: uuid::Uuid,
    pub messages: Vec<ChatMessage>,
    pub state: ChatState,
    pub new_message_count: usize,
}

impl Chat {
    pub(crate) fn add_message(&mut self, message: ChatMessage) {
        if !message.role().is_user() {
            self.new_message_count += 1;
        }
        self.messages.push(message);
    }

    pub fn transition(&mut self, state: ChatState) {
        self.state = state;
    }

    pub fn in_state(&self, state: ChatState) -> bool {
        self.state == state
    }
}

#[derive(Debug, Clone, Copy, Default, strum::EnumIs, PartialEq)]
pub enum ChatState {
    Loading,
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
            new_message_count: 0,
        }
    }
}
