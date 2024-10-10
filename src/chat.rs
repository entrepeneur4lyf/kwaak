use crate::chat_message::ChatMessage;

#[derive(Debug, Clone)]
pub struct Chat {
    pub name: String,
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
            name: String::new(),
            uuid: uuid::Uuid::new_v4(),
            messages: Vec::new(),
            state: ChatState::default(),
        }
    }
}
