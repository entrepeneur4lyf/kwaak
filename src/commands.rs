use anyhow::Result;
use tokio::sync::mpsc;

use crate::app::{App, UIEvent};

/// Commands represent concrete actions from a user or in the backend
#[derive(Debug, PartialEq, Eq, strum_macros::EnumString, strum_macros::Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Quit,
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

// Commands always flow via the CommandHandler
pub struct CommandHandler {
    rx: mpsc::UnboundedReceiver<Command>,
    #[allow(dead_code)]
    tx: mpsc::UnboundedSender<Command>,
    ui_tx: mpsc::UnboundedSender<UIEvent>,
}

impl CommandHandler {
    pub fn start_with_ui_app(app: &mut App) -> tokio::task::JoinHandle<()> {
        let (tx, rx) = mpsc::unbounded_channel();
        let ui_tx = app.ui_tx.clone();
        app.command_tx = Some(tx.clone());

        let handler = CommandHandler { rx, tx, ui_tx };

        handler.start()
    }

    fn start(mut self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(cmd) = self.rx.recv().await {
                // For now just forward all commands to the UI
                self.handle_command(cmd).await.unwrap();
            }
        })
    }

    async fn handle_command(&self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Quit => self.ui_tx.send(UIEvent::Command(cmd))?,
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
