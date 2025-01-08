#[cfg(test)]
mod tests {
    use crate::chat::Chat;
    use crate::chat_message::{ChatMessage, ChatRole};
    use crate::frontend::chat_mode::{format_chat_message, message_styles};
    use ratatui::prelude::*;

    #[test]
    fn test_user_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage::default(ChatRole::User, "Hello, this is a user message.", None);
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "▶ ";
        let expected_style = message_styles::USER;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }

    #[test]
    fn test_assistant_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage::default(
            ChatRole::Assistant,
            "Hello, this is an assistant message.",
            None,
        );
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "✦ ";
        let expected_style = message_styles::ASSISTANT;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }

    #[test]
    fn test_system_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage::default(ChatRole::System, "This is a system message.", None);
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "ℹ ";
        let expected_style = message_styles::SYSTEM;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }
}
