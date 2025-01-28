use crate::chat_message::ChatMessage;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone, Default, PartialEq)]
pub enum ChatState {
    Loading,
    LoadingWithMessage(String),
    Ready,
}

#[derive(Clone, Default, PartialEq)]
pub struct Chat {
    pub name: String,
    pub uuid: Uuid,
    pub branch_name: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub state: ChatState,
    pub new_message_count: usize,
    pub completed_tool_call_ids: HashSet<String>,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub num_lines: usize,
}

impl Chat {
    #[must_use]
    pub fn is_loading(&self) -> bool {
        matches!(
            self.state,
            ChatState::Loading | ChatState::LoadingWithMessage(_)
        )
    }
    
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
        self.new_message_count += 1;
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            name: "Chat".to_string(),
            uuid: Uuid::new_v4(),
            branch_name: None,
            messages: Vec::new(),
            state: ChatState::Ready,
            new_message_count: 0,
            completed_tool_call_ids: HashSet::new(),
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
            num_lines: 0,
        }
    }
}
