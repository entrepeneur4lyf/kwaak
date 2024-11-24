use derive_builder::Builder;
use uuid::Uuid;

/// Represents a chat message that can be stored in a [`Chat`]
///
/// Messages are expected to be formatted strings and are displayed as-is. Markdown is rendered
/// using `tui-markdown`.
#[derive(Clone, Default, Builder)]
#[builder(setter(into, strip_option), build_fn(skip))]
pub struct ChatMessage {
    role: ChatRole,
    content: String,
    uuid: Option<Uuid>,
}

// Debug with truncated content
impl std::fmt::Debug for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChatMessage")
            .field("role", &self.role)
            .field(
                "content",
                &self.content[..std::cmp::min(10, self.content.len())].to_string(),
            )
            .field("uuid", &self.uuid)
            .finish()
    }
}

#[derive(
    Debug, Clone, Copy, Default, strum::EnumString, strum::Display, strum::AsRefStr, strum::EnumIs,
)]
pub enum ChatRole {
    User,
    #[default]
    System,
    Command,
    Assistant,
    Tool,
}

impl ChatMessage {
    pub fn new_user(msg: impl Into<String>) -> ChatMessageBuilder {
        ChatMessageBuilder::default()
            .role(ChatRole::User)
            .content(msg.into())
            .to_owned()
    }

    pub fn new_system(msg: impl Into<String>) -> ChatMessageBuilder {
        ChatMessageBuilder::default()
            .role(ChatRole::System)
            .content(msg.into())
            .to_owned()
    }

    pub fn new_command(cmd: impl Into<String>) -> ChatMessageBuilder {
        ChatMessageBuilder::default()
            .role(ChatRole::Command)
            .content(cmd.into().to_string())
            .to_owned()
    }

    pub fn new_assistant(msg: impl Into<String>) -> ChatMessageBuilder {
        ChatMessageBuilder::default()
            .role(ChatRole::Assistant)
            .content(msg.into())
            .to_owned()
    }

    pub fn new_tool(msg: impl Into<String>) -> ChatMessageBuilder {
        ChatMessageBuilder::default()
            .role(ChatRole::Tool)
            .content(msg.into())
            .to_owned()
    }

    pub fn uuid(&self) -> Option<Uuid> {
        self.uuid
    }
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn role(&self) -> &ChatRole {
        &self.role
    }

    pub fn with_uuid(self, uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            ..self
        }
    }
}

impl From<swiftide::chat_completion::ChatMessage> for ChatMessage {
    fn from(msg: swiftide::chat_completion::ChatMessage) -> Self {
        match msg {
            swiftide::chat_completion::ChatMessage::System(msg) => {
                ChatMessage::new_system(msg).build()
            }
            swiftide::chat_completion::ChatMessage::User(msg) => ChatMessage::new_user(msg).build(),
            swiftide::chat_completion::ChatMessage::Assistant(msg) => {
                ChatMessage::new_assistant(msg).build()
            }
            swiftide::chat_completion::ChatMessage::ToolCall(tool_call) => {
                ChatMessage::new_tool(format!(
                    "calling tool `{}` with `{}`",
                    tool_call.name(),
                    tool_call.args().unwrap_or("no arguments")
                ))
                .build()
            }
            swiftide::chat_completion::ChatMessage::ToolOutput(tool_call, _) => {
                ChatMessage::new_tool(format!(
                    "tool `{}` with `{}` completed",
                    tool_call.name(),
                    tool_call.args().unwrap_or("no arguments")
                ))
                .build()
            }
        }
    }
}

impl ChatMessageBuilder {
    // Building is infallible
    pub fn build(&mut self) -> ChatMessage {
        ChatMessage {
            content: self.content.clone().unwrap_or_default(),
            uuid: self.uuid.unwrap_or_default(),
            role: self.role.unwrap_or_default(),
        }
    }
}

fn format_tool_call(tool_call: &swiftide::chat_completion::ToolCall) -> String {
    // If args, parse them as a json value, then if its just one, render only the value, otherwise
    // limit the output to 20 characters
    let formatted_args = tool_call.args().map_or("no arguments".to_string(), |args| {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(args) {
            if let Some(obj) = parsed.as_object() {
                if obj.keys().count() == 1 {
                    let key = obj.keys().next().unwrap();
                    let val = obj[key].as_str().unwrap_or_default();

                    if val.len() > 20 {
                        return format!("{} ...", &val[..20]);
                    }

                    return val.to_string();
                }
                return args.to_string();
            }

            args.to_string()
        } else {
            args.to_string()
        }
    });

    format!(
        "calling tool `{}` with `{}`",
        tool_call.name(),
        formatted_args
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use swiftide::chat_completion::ToolCall;

    #[test]
    fn test_format_tool_call_no_arguments() {
        let tool_call = ToolCall::builder()
            .name("test_tool")
            .id("test_id")
            .build()
            .unwrap();

        let result = format_tool_call(&tool_call);
        assert_eq!(result, "calling tool `test_tool` with `no arguments`");
    }

    #[test]
    fn test_format_tool_call_single_argument() {
        let tool_call = ToolCall::builder()
            .name("test_tool")
            .id("test_id")
            .args(r#"{"key": "value"}"#.to_string())
            .build()
            .unwrap();

        let result = format_tool_call(&tool_call);
        assert_eq!(result, "calling tool `test_tool` with `value`");
    }

    #[test]
    fn test_format_tool_call_multiple_arguments() {
        let tool_call = ToolCall::builder()
            .name("test_tool")
            .id("test_id")
            .args(r#"{"key1": "value1", "key2": "value2"}"#.to_string())
            .build()
            .unwrap();

        let result = format_tool_call(&tool_call);
        assert_eq!(
            result,
            "calling tool `test_tool` with `{\"key1\": \"value1\", \"key2\": \"value2\"}`"
        );
    }

    #[test]
    fn test_format_tool_call_invalid_json() {
        let tool_call = ToolCall::builder()
            .name("test_tool")
            .id("test_id")
            .args("invalid json".to_string())
            .build()
            .unwrap();

        let result = format_tool_call(&tool_call);
        assert_eq!(result, "calling tool `test_tool` with `invalid json`");
    }
}
