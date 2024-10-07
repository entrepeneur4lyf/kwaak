use anyhow::Result;
use tokio::{sync::mpsc, task};

use crate::{
    chat_message::ChatMessage,
    frontend::{App, UIEvent},
    indexing, query,
    repository::Repository,
};

/// Commands represent concrete actions from a user or in the backend
///
/// By default all commands can be triggered from the ui like `/<command>`
#[derive(
    Debug,
    PartialEq,
    Eq,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
    strum_macros::IntoStaticStr,
    Clone,
)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Quit,
    ShowConfig,
    IndexRepository,
    // Currently just dispatch a user message command and answer the query
    // Later, perhaps main a 'chat', add message to that chat, and then send
    // the whole thing
    Chat(String),
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, strum::ParseError> {
        // FIXME: Will break on current Chat variant
        if let Some(input) = input.strip_prefix('/') {
            input.parse()
        } else {
            Err(strum::ParseError::VariantNotFound)
        }
    }
}

/// Commands always flow via the `CommandHandler`
pub struct CommandHandler {
    /// Receives commands
    rx: mpsc::UnboundedReceiver<Command>,
    #[allow(dead_code)]
    /// Sends commands
    tx: mpsc::UnboundedSender<Command>,
    /// Sends `UIEvents` to the connected frontend
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
        task::spawn(async move {
            while let Some(cmd) = self.rx.recv().await {
                if let Err(error) = self.handle_command(cmd.clone()).await {
                    tracing::error!(?error, %cmd, "Failed to handle command {cmd} with error {error:#}");
                    self.send_system_message(format!("Failed to handle command: {error:#}"));
                }
            }
        })
    }

    fn send_system_message(&self, msg: impl Into<String>) {
        let msg = ChatMessage::new_system(msg);
        let _ = self.ui_tx.send(msg.into());
    }

    /// TODO: Most commands should probably be handled in a tokio task
    /// Maybe generalize tasks to make ui updates easier?
    async fn handle_command(&self, cmd: Command) -> Result<()> {
        let now = std::time::Instant::now();
        match cmd {
            Command::IndexRepository => indexing::index_repository(&self.repository).await?,
            Command::ShowConfig => {
                self.send_system_message(toml::to_string_pretty(self.repository.config())?);
            }
            Command::Chat(ref msg) => {
                let response = query::query(&self.repository, msg).await?;
                tracing::info!(%response, "Chat message received, sending to frontend");
                let response = ChatMessage::new_system(response);

                self.ui_tx.send(response.into())?;
            }
            // Anything else we forward to the UI
            _ => self.ui_tx.send(UIEvent::Command(cmd.clone()))?,
        }
        let elapsed = now.elapsed();
        self.send_system_message(format!(
            "Command {cmd} successful in {} seconds",
            elapsed.as_secs_f64()
        ));

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
