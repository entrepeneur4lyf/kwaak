use anyhow::Result;
use tokio::sync::mpsc;

use crate::{
    app::{App, UIEvent},
    chat_message::ChatMessage,
    config::Config,
    indexing,
    repository::{self, Repository},
};

/// Commands represent concrete actions from a user or in the backend
///
/// By default all commands can be triggered from the ui like `/<command>`
#[derive(Debug, PartialEq, Eq, strum_macros::EnumString, strum_macros::Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Quit,
    ShowConfig,
    IndexRepository,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, strum::ParseError> {
        if let Some(input) = input.strip_prefix('/') {
            input.parse()
        } else {
            Err(strum::ParseError::VariantNotFound)
        }
    }
}

/// Commands always flow via the CommandHandler
pub struct CommandHandler {
    /// Receives commands
    rx: mpsc::UnboundedReceiver<Command>,
    #[allow(dead_code)]
    /// Sends commands
    tx: mpsc::UnboundedSender<Command>,
    /// Sends UIEvents to the connected frontend
    ui_tx: mpsc::UnboundedSender<UIEvent>,
    /// Repository to interact with
    repository: Repository,
}

impl CommandHandler {
    pub fn start_with_ui_app(app: &mut App, repository: Repository) -> tokio::task::JoinHandle<()> {
        let (tx, rx) = mpsc::unbounded_channel();
        let ui_tx = app.ui_tx.clone();
        app.command_tx = Some(tx.clone());

        let handler = CommandHandler {
            rx,
            tx,
            ui_tx,
            repository,
        };

        handler.start()
    }

    fn start(mut self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(cmd) = self.rx.recv().await {
                self.handle_command(cmd).await.unwrap();
            }
        })
    }

    /// TODO: Most commands should probably be handled in a tokio task
    /// Maybe generalize tasks to make ui updates easier?
    async fn handle_command(&self, cmd: Command) -> Result<()> {
        match cmd {
            Command::IndexRepository => indexing::index_repository(&self.repository).await?,
            Command::ShowConfig => {
                self.ui_tx
                    .send(UIEvent::ChatMessage(ChatMessage::new_system(
                        toml::to_string_pretty(self.repository.config())?,
                    )))?
            }
            // Anything else we forward to the UI
            _ => self.ui_tx.send(UIEvent::Command(cmd))?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_from_str() {
        assert_eq!("quit".parse(), Ok(Command::Quit));
    }

    #[test]
    fn test_command_to_string() {
        assert_eq!(Command::Quit.to_string(), "quit");
    }

    #[test]
    fn test_parse_str_with_prefix() {
        assert_eq!(Command::parse("/quit"), Ok(Command::Quit));
    }
}
