use derive_builder::Builder;
use uuid::Uuid;

/// Represents a chat message that can be stored in a [`Chat`]
/// TODO: Should we just use swiftide chat messages to avoid confusion?
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
