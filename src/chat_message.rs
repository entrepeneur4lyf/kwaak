use derive_builder::Builder;
use uuid::Uuid;

use crate::commands::Command;

/// Represents a chat message that can be stored in a [`Chat`]
#[derive(Debug, Clone, Default, Builder)]
#[builder(setter(into, strip_option), build_fn(skip))]
pub struct ChatMessage {
    role: ChatRole,
    content: String,
    uuid: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, Default, strum::EnumString, strum::Display, strum::AsRefStr)]
pub enum ChatRole {
    User,
    #[default]
    System,
    Command,
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

    pub fn new_command(cmd: impl Into<Command>) -> ChatMessageBuilder {
        ChatMessageBuilder::default()
            .role(ChatRole::Command)
            .content(cmd.into().to_string())
            .to_owned()
    }

    pub fn uuid(&self) -> Option<Uuid> {
        self.uuid
    }
    pub fn message(&self) -> &str {
        &self.content
    }

    pub fn role(&self) -> &ChatRole {
        &self.role
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
