use ratatui::prelude::*;
use serde_json::json;

// Import from accessible submodules instead of direct crate root imports
use kwaak::{
    chat::Chat,
    chat_message::{ChatMessage, ChatRole},
    frontend::chat_mode::message_formatting::{
        format_chat_message, format_tool_call, get_style_and_prefix, pretty_format_tool,
    },
};

// Dummy ToolCall for test instantiations
struct DummyToolCall {
    name: String,
    args: Option<String>,
}

impl DummyToolCall {
    fn new(name: &str, args: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            args,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn args(&self) -> Option<&str> {
        self.args.as_deref()
    }
}

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
    let chat = Chat::default(); // Assuming default or placeholder to instantiate Demo Chat
    let message = ChatMessage::new_user("Test message content");
    let formatted_message = format_chat_message(&chat, &message);
    // Check if the formatted message contains the correct symbol and user message
    assert!(formatted_message
        .lines
        .first()
        .unwrap()
        .to_string()
        .contains("▶ "));
    assert!(formatted_message
        .to_string()
        .contains("Test message content"));
}

#[test]
fn test_format_tool_call() {
    let tool_call = DummyToolCall::new("test_tool", Some(json!({ "arg1": "value1" }).to_string()));
    let formatted_tool_call = format_tool_call(&tool_call);
    assert!(formatted_tool_call.contains("calling tool `test_tool` with `value1`"));
}

#[test]
fn test_pretty_format_tool() {
    let tool_call = DummyToolCall::new(
        "shell_command",
        Some(json!({ "cmd": "ls -al" }).to_string()),
    );
    let formatted_tool_call = pretty_format_tool(&tool_call).unwrap();
    assert_eq!(formatted_tool_call, "running shell command `ls -al`");
}
