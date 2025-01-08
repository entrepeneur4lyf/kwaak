#[cfg(test)]
mod tests {
    use crate::chat::Chat;
    use crate::chat_message::ChatMessage;
    use crate::frontend::chat_mode::message_formatting::format_chat_message;

    #[test]
    fn test_user_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage::new_user("Hello, this is a user message.").build();
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "▶ ";
        let expected_style = crate::frontend::chat_mode::message_formatting::message_styles::USER;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }

    #[test]
    fn test_assistant_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage::new_assistant("Hello, this is an assistant message.").build();
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "✦ ";
        let expected_style =
            crate::frontend::chat_mode::message_formatting::message_styles::ASSISTANT;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }

    #[test]
    fn test_system_message_formatting() {
        let chat = Chat::default();
        let message = ChatMessage::new_system("This is a system message.").build();
        let formatted_text = format_chat_message(&chat, &message);
        let expected_prefix = "ℹ ";
        let expected_style = crate::frontend::chat_mode::message_formatting::message_styles::SYSTEM;

        assert_eq!(formatted_text.lines[0].spans[0].content, expected_prefix);
        assert_eq!(formatted_text.lines[0].spans[0].style, expected_style);
    }
}
