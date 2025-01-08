#[cfg(test)]
mod tests {
    use crate::chat::Chat;
    use crate::chat_message::{ChatMessage, ChatRole};
    use crate::frontend::chat_mode::message_formatting::{
        format_chat_message, get_style_and_prefix,
    };
    use ratatui::prelude::*;
    use ratatui::text::{Span, Spans, Text};

    #[test]
    fn test_user_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage {
            role: ChatRole::User,
            content: String::from("Hello, this is a user message."),
            ..Default::default()
        };
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "▶ ";
        let expected_style = message_styles::USER;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }

    #[test]
    fn test_assistant_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage {
            role: ChatRole::Assistant,
            content: String::from("Hello, this is an assistant message."),
            ..Default::default()
        };
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "✦ ";
        let expected_style = message_styles::ASSISTANT;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }

    #[test]
    fn test_system_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage {
            role: ChatRole::System,
            content: String::from("This is a system message."),
            ..Default::default()
        };
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "ℹ ";
        let expected_style = message_styles::SYSTEM;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }
}
