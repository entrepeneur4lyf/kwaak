use anyhow::Result;
use derive_builder::Builder;
use tokio::{sync::mpsc, task};
use uuid::Uuid;

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
    strum_macros::EnumIs,
    Clone,
)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Quit {
        uuid: Uuid,
    },
    ShowConfig {
        uuid: Uuid,
    },
    IndexRepository {
        uuid: Uuid,
    },
    /// Default when no command is provided
    #[strum(disabled)]
    Chat {
        uuid: Uuid,
        message: String,
    },
}

impl Command {
    pub fn uuid(&self) -> Uuid {
        match self {
            Command::Quit { uuid }
            | Command::ShowConfig { uuid }
            | Command::IndexRepository { uuid }
            | Command::Chat { uuid, .. } => *uuid,
        }
    }

    pub fn with_uuid(self, uuid: Uuid) -> Self {
        match self {
            Command::Quit { .. } => Command::Quit { uuid },
            Command::ShowConfig { .. } => Command::ShowConfig { uuid },
            Command::IndexRepository { .. } => Command::IndexRepository { uuid },
            Command::Chat { message, .. } => Command::Chat { uuid, message },
        }
    }
}

impl Command {
    pub fn parse(input: &str, uuid: Option<Uuid>) -> Result<Self, strum::ParseError> {
        // FIXME: Will break on current Chat variant
        if let Some(input) = input.strip_prefix('/') {
            input
                .parse()
                .map(|cmd: Command| cmd.with_uuid(uuid.unwrap_or_default()))
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
    ui_tx: Option<mpsc::UnboundedSender<UIEvent>>,
    /// Repository to interact with
    repository: Repository,
}

impl CommandHandler {
    pub fn from_repository(repository: impl Into<Repository>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        CommandHandler {
            rx,
            tx,
            ui_tx: None,
            repository: repository.into(),
        }
    }

    pub fn register_ui(&mut self, app: &mut App) {
        self.ui_tx = Some(app.ui_tx.clone());
        app.command_tx = Some(self.tx.clone());
    }

    pub fn start(mut self) -> tokio::task::JoinHandle<()> {
        task::spawn(async move {
            while let Some(cmd) = self.rx.recv().await {
                if let Err(error) = self.handle_command(&cmd).await {
                    tracing::error!(?error, %cmd, "Failed to handle command {cmd} with error {error:#}");
                    self.send_ui_event(
                        ChatMessage::new_system(format!("Failed to handle command: {error:#}"))
                            .uuid(cmd.uuid())
                            .to_owned(),
                    );
                }
            }
        })
    }

    fn send_ui_event(&self, msg: impl Into<UIEvent>) {
        if let Err(error) = self
            .ui_tx
            .as_ref()
            .expect("Expected a registered ui")
            .send(msg.into())
        {
            tracing::error!(?error, "Failed to send UI event");
        }
    }

    /// TODO: Most commands should probably be handled in a tokio task
    /// Maybe generalize tasks to make ui updates easier?
    async fn handle_command(&self, cmd: &Command) -> Result<()> {
        let now = std::time::Instant::now();

        #[allow(clippy::match_wildcard_for_single_variants)]
        match cmd {
            Command::IndexRepository { .. } => indexing::index_repository(&self.repository).await?,
            Command::ShowConfig { uuid } => {
                self.send_ui_event(
                    ChatMessage::new_system(toml::to_string_pretty(self.repository.config())?)
                        .uuid(*uuid)
                        .to_owned(),
                );
            }
            Command::Chat { uuid, ref message } => {
                let response = query::query(&self.repository, message).await?;
                tracing::info!(%response, "Chat message received, sending to frontend");
                let response = ChatMessage::new_system(response).uuid(*uuid).to_owned();

                self.send_ui_event(response);
            }
            // Anything else we forward to the UI
            _ => self.send_ui_event(cmd.clone()),
        }
        let elapsed = now.elapsed();
        self.send_ui_event(
            ChatMessage::new_system(format!(
                "Command {cmd} successful in {} seconds",
                elapsed.as_secs_f64()
            ))
            .uuid(cmd.uuid()),
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_from_str() {
        let cmd: Command = "quit".parse().unwrap();
        assert!(cmd.is_quit());
    }

    #[test]
    fn test_command_to_string() {
        assert_eq!(
            Command::Quit {
                uuid: Uuid::new_v4()
            }
            .to_string(),
            "quit"
        );
    }

    #[test]
    fn test_parse_str_with_prefix() {
        let cmd = Command::parse("/quit", None).unwrap();
        assert!(cmd.is_quit());
    }

    #[test]
    fn test_parse_str_with_prefix_and_uid() {
        let uuid = Uuid::new_v4();
        let cmd = Command::parse("/quit", Some(uuid)).unwrap();

        assert!(cmd.is_quit());
        assert_eq!(cmd.uuid(), uuid);
    }
}
