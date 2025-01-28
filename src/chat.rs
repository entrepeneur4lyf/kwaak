use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Chat {
    pub name: String,
    pub uuid: uuid::Uuid,
    pub branch_name: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub state: ChatState,
    pub new_message_count: usize,
    pub completed_tool_call_ids: HashSet<String>,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub num_lines: usize,
    pub auto_tailing_enabled: bool,  // New field for auto-tailing
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            name: String::new(),
            uuid: uuid::Uuid::new_v4(),
            branch_name: None,
            messages: Vec::new(),
            state: ChatState::default(),
            new_message_count: 0,
            completed_tool_call_ids: HashSet::new(),
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
            num_lines: 0,
            auto_tailing_enabled: true,  // Auto-tailing enabled by default
        }
    }
}
