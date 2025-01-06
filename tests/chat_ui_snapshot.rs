use insta::assert_snapshot;
use kwaak::chat::{Chat, ChatState};
use kwaak::chat_message::ChatMessage;
use uuid::Uuid;

#[test]
fn test_chat_ui_snapshot() {
    // Create a new chat instance
    let mut chat = Chat {
        name: "Test Chat".to_string(),
        uuid: Uuid::new_v4(),
        messages: vec![ChatMessage::new_user("Hello, world!").build()], // Finalize ChatMessage
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
    // chat.add_message(ChatMessage::new_user("Another message").build());
    chat.transition(ChatState::Ready);

    // Take a snapshot of the modified chat state
    assert_snapshot!(format!("{:?}", chat));
}

// Removed to resolve issues with inherent implementation and derive usage

#[derive(Debug, Clone, Default)]
pub enum CustomChatRole {
    #[default]
    User,
}
