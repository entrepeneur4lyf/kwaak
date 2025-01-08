use ratatui::prelude::*;
use serde_json::json;

use crate::chat::Chat;
use crate::chat_message::{ChatMessage, ChatRole};
use crate::frontend::chat_mode::message_formatting::{
    format_chat_message, format_tool_call, get_style_and_prefix, pretty_format_tool,
};
use swiftide::chat_completion::ToolCall;

#[test]
fn test_get_style_and_prefix() {
    assert_eq!(get_style_and_prefix(&ChatRole::User).0, "▶ ");
    assert_eq!(get_style_and_prefix(&ChatRole::Assistant).0, "✦ ");
    assert_eq!(get_style_and_prefix(&ChatRole::System).0, "ℹ ");
    assert_eq!(get_style_and_prefix(&ChatRole::Tool).0, "⚙ ");
    assert_eq!(get_style_and_prefix(&ChatRole::Command).0, "» ");
}

#[test]
fn test_format_chat_message() {
    let chat = Chat::new(vec![]);
    let message = ChatMessage::new_user("Test message content");
    let formatted_message = format_chat_message(&chat, &message);
    // Check if the formatted message contains the symbol for a user
    assert!(formatted_message
        .lines
        .first()
        .unwrap()
        .to_string()
        .contains("▶ "));
    // Check if the formatted message contains the original message content
    assert!(formatted_message
        .to_string()
        .contains("Test message content"));
}

#[test]
fn test_format_tool_call() {
    let tool_call = ToolCall::new("test_tool", None, json!({ "arg1": "value1" }).to_string());
    let formatted_tool_call = format_tool_call(&tool_call);
    assert!(formatted_tool_call.contains("calling tool `test_tool` with `value1`"));
}

#[test]
fn test_pretty_format_tool() {
    let tool_call = ToolCall::new(
        "shell_command",
        None,
        json!({ "cmd": "ls -al" }).to_string(),
    );
    let formatted_tool_call = pretty_format_tool(&tool_call).unwrap();
    assert_eq!(formatted_tool_call, "running shell command `ls -al`");
}
