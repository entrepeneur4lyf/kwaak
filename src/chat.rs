use crate::chat_message::ChatMessage;

// Ensure you define or include necessary structures for Chat and other modules

pub struct Chat {
    pub messages: Vec<ChatMessage>,
    pub auto_tailing_enabled: bool,
    pub message_scroll_offset: usize,
}

// You'd need other implementations and imports such as ChatState based on your project
