#[cfg(test)]
mod chat_tests {
    use super::*;
    use crate::chat_message::{ChatMessage, MessageRole};

    #[test]
    fn test_default_chat_initialization() {
        let chat = Chat::default();
        assert_eq!(chat.name, "Chat");
        assert_eq!(chat.messages.len(), 0);
        assert_eq!(chat.state, ChatState::Ready);
        assert_eq!(chat.new_message_count, 0);
        assert!(chat.completed_tool_call_ids.is_empty());
    }

    #[test]
    fn test_chat_add_user_message() {
        let mut chat = Chat::default();
        let message = ChatMessage::new(MessageRole::User, "Hello World");

        chat.add_message(message.clone());

        assert_eq!(chat.messages.len(), 1);
        assert_eq!(chat.messages[0], message);
        assert_eq!(chat.new_message_count, 0); // user message shouldn't increment
    }

    #[test]
    fn test_chat_add_tool_message() {
        let mut chat = Chat::default();
        let message = ChatMessage::new_tool("tool-id", "completed");

        chat.add_message(message);

        assert_eq!(chat.messages.len(), 0); // tool messages are not added
        assert_eq!(chat.new_message_count, 0);
        assert!(chat.is_tool_call_completed("tool-id"));
    }

    #[test]
    fn test_chat_transition() {
        let mut chat = Chat::default();
        chat.transition(ChatState::Loading);
        assert!(chat.is_loading());

        chat.transition(ChatState::Ready);
        assert!(!chat.is_loading());
    }
}
