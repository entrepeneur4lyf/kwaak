use kwaak::chat::Chat;
use kwaak::chat_message::{ChatMessage, ChatRole};
use kwaak::frontend::chat_mode::format_chat_message;
use ratatui::style::{Color, Modifier};

#[test]
fn test_user_message_formatting() {
    let chat = Chat::default(); // Assuming the default constructor is available
    let message = ChatMessage::new_user("User message").build();
    let formatted_message = format_chat_message(&chat, &message);

    // Check the first line's prefix and style
    if let Some(first_line) = formatted_message.lines.first() {
        if let Some(first_span) = first_line.spans.first() {
            assert_eq!(first_span.content, "▶ ");
            assert_eq!(first_span.style.fg, Some(Color::Cyan));
            assert!(first_span.style.add_modifier.contains(Modifier::ITALIC));
        }
    }
}

#[test]
fn test_assistant_message_formatting() {
    let chat = Chat::default();
    let message = ChatMessage::new_assistant("Assistant message").build();
    let formatted_message = format_chat_message(&chat, &message);

    if let Some(first_line) = formatted_message.lines.first() {
        if let Some(first_span) = first_line.spans.first() {
            assert_eq!(first_span.content, "✦ ");
            assert_eq!(first_span.style.fg, Some(Color::Rgb(200, 160, 255)));
            assert!(first_span.style.add_modifier.contains(Modifier::BOLD));
        }
    }
}

#[test]
fn test_system_message_formatting() {
    let chat = Chat::default();
    let message = ChatMessage::new_system("System message").build();
    let formatted_message = format_chat_message(&chat, &message);

    if let Some(first_line) = formatted_message.lines.first() {
        if let Some(first_span) = first_line.spans.first() {
            assert_eq!(first_span.content, "ℹ ");
            assert_eq!(first_span.style.fg, Some(Color::DarkGray));
            assert!(first_span.style.add_modifier.contains(Modifier::DIM));
        }
    }
}

#[test]
fn test_command_message_formatting() {
    let chat = Chat::default();
    let message = ChatMessage::new_command("Command message").build();
    let formatted_message = format_chat_message(&chat, &message);

    if let Some(first_line) = formatted_message.lines.first() {
        if let Some(first_span) = first_line.spans.first() {
            assert_eq!(first_span.content, "» ");
            assert_eq!(first_span.style.fg, Some(Color::LightMagenta));
            assert!(first_span.style.add_modifier.contains(Modifier::BOLD));
        }
    }
}
