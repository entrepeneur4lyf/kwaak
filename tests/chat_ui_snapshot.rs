use crate::chat::{Chat, ChatState};
use crate::chat_message::ChatMessage;
use insta::assert_snapshot;
use uuid::Uuid;

#[test]
fn test_chat_ui_snapshot() {
    // Create a new chat instance
    let mut chat = Chat {
        name: "Test Chat".to_string(),
        uuid: Uuid::new_v4(),
        messages: vec![ChatMessage::new_user_message("Hello, world!")],
        state: ChatState::Ready,
        new_message_count: 1,
        completed_tool_call_ids: Default::default(),
        vertical_scroll_state: Default::default(),
        vertical_scroll: 0,
        num_lines: 5,
    };

    // Modify chat instance to represent the state when loading
    chat.transition(ChatState::Loading);

    // Take a snapshot of the initial chat state
    assert_snapshot!(format!("{:?}", chat));

    // Add a new message and transition to Ready state
    chat.add_message(ChatMessage::new_user_message("Another message"));
    chat.transition(ChatState::Ready);

    // Take a snapshot of the modified chat state
    assert_snapshot!(format!("{:?}", chat));
}

impl ChatMessage {
    pub fn new_user_message(content: &str) -> Self {
        ChatMessage {
            content: content.to_string(),
            role: ChatRole::User,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum ChatRole {
    #[default]
    User,
}
