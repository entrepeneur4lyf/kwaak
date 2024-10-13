
use uuid::Uuid;

use crate::commands::Command;

#[derive(
    Debug,
    Clone,
    Copy,
    strum_macros::Display,
    strum_macros::EnumIs,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
#[strum(serialize_all = "snake_case")]
pub enum UserInputCommand {
    Quit,
    ShowConfig,
    IndexRepository,
    NextChat,
    NewChat,
}

impl UserInputCommand {
    pub fn to_command(self, uuid: Uuid) -> Option<Command> {
        match self {
            UserInputCommand::Quit => Some(Command::Quit { uuid }),
            UserInputCommand::ShowConfig => Some(Command::ShowConfig { uuid }),
            UserInputCommand::IndexRepository => Some(Command::IndexRepository { uuid }),
            _ => None,
        }
    }
}
