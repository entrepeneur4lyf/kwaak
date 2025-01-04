use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Result;
use swiftide::agents::Agent;
use tokio::{
    sync::{mpsc, Mutex, RwLock},
    task::{self, JoinHandle},
};
use uuid::Uuid;

use crate::{
    agent,
    chat_message::ChatMessage,
    frontend::{App, UIEvent},
    indexing,
    repository::Repository,
};

/// Commands represent concrete actions from a user or in the backend
#[derive(
    Debug,
    PartialEq,
    Eq,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::IntoStaticStr,
    strum_macros::EnumIs,
    Clone,
)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Quit { uuid: Uuid },
    ShowConfig { uuid: Uuid },
    IndexRepository { uuid: Uuid },
    StopAgent { uuid: Uuid },
    Chat { uuid: Uuid, message: String },
}

pub enum CommandResponse {
    Chat(ChatMessage),
    ActivityUpdate(Uuid, String),
}

#[derive(Debug)]
pub struct CommandResponder {
    tx: mpsc::UnboundedSender<CommandResponse>,
    rx: Option<mpsc::UnboundedReceiver<CommandResponse>>,
    uuid: Uuid,
}

impl CommandResponder {
    pub fn send_system_message(&self, message: &str) {
        self.send_message(ChatMessage::new_system(message).build());
    }

    pub fn send_message<M: Into<ChatMessage>>(&self, msg: M) {
        let _ = self.tx.send(CommandResponse::Chat(msg.into().with_uuid(self.uuid)));
    }

    pub fn send_update(&self, state: &str) {
        let _ = self.tx.send(CommandResponse::ActivityUpdate(self.uuid, state.to_string()));
    }

    pub async fn recv(&mut self) -> Option<CommandResponse> {
        let rx = self.rx.as_mut().expect("Receiver must exist");
        rx.recv().await
    }

    pub fn with_uuid(self, uuid: Uuid) -> Self {
        CommandResponder {
            tx: self.tx,
            rx: self.rx,
            uuid,
        }
    }
}

impl Default for CommandResponder {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        CommandResponder {
            tx,
            rx: Some(rx),
            uuid: Uuid::default(),
        }
    }
}

impl Clone for CommandResponder {
    fn clone(&self) -> Self {
        CommandResponder {
            tx: self.tx.clone(),
            rx: None,
            uuid: self.uuid,
        }
    }
}

impl From<ChatMessage> for CommandResponse {
    fn from(msg: ChatMessage) -> Self {
        CommandResponse::Chat(msg)
    }
}

impl Command {
    pub fn uuid(&self) -> Uuid {
        match self {
            Command::Quit { uuid }
            | Command::StopAgent { uuid }
            | Command::ShowConfig { uuid }
            | Command::IndexRepository { uuid }
            | Command::Chat { uuid, .. } => *uuid,
        }
    }

    pub fn with_uuid(self, uuid: Uuid) -> Self {
        match self {
            Command::StopAgent { .. } => Command::StopAgent { uuid },
            Command::Quit { .. } => Command::Quit { uuid },
            Command::ShowConfig { .. } => Command::ShowConfig { uuid },
            Command::IndexRepository { .. } => Command::IndexRepository { uuid },
            Command::Chat { message, .. } => Command::Chat { uuid, message },
        }
    }
}

pub struct CommandHandler {
    rx: Option<mpsc::UnboundedReceiver<Command>>,
    tx: mpsc::UnboundedSender<Command>,
    ui_tx: Option<mpsc::UnboundedSender<UIEvent>>,
    repository: Arc<Repository>,
    agents: Arc<RwLock<HashMap<Uuid, RunningAgent>>>,
}

#[derive(Clone)]
struct RunningAgent {
    agent: Arc<Mutex<Agent>>,
    handle: Arc<tokio::task::JoinHandle<()>>,
}

impl RunningAgent {
    pub async fn query(&self, query: &str) -> Result<()> {
        self.agent.lock().await.query(query).await
    }

    pub async fn stop(&self) {
        self.agent.lock().await.stop();
    }
}

impl CommandHandler {
    pub fn from_repository(repository: impl Into<Repository>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        CommandHandler {
            rx: Some(rx),
            tx,
            ui_tx: None,
            repository: Arc::new(repository.into()),
            agents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_ui(&mut self, app: &mut App) {
        self.ui_tx = Some(app.ui_tx.clone());
        app.command_tx = Some(self.tx.clone());
    }

    pub fn start(mut self) -> tokio::task::JoinHandle<()> {
        let repository = Arc::clone(&self.repository);
        let ui_tx = self.ui_tx.clone().expect("Expected a registered UI transmitter");
        let mut rx = self.rx.take().expect("Expected command receiver");
        let this_handler = Arc::new(self);

        task::spawn(async move {
            let mut joinset = tokio::task::JoinSet::new();

            while let Some(cmd) = rx.recv().await {
                if cmd.is_quit() {
                    tracing::warn!("Backend received quit command, shutting down");
                    joinset.shutdown().await;
                    tracing::warn!("Backend shutdown complete");
                    break;
                }

                // Ensure commands are properly handled even if task processing is interrupted
                let repository = Arc::clone(&repository);
                let ui_tx = ui_tx.clone();
                let this_handler = Arc::clone(&this_handler);
                joinset.spawn(async move {
                    let result = this_handler.handle_command(&repository, &ui_tx, &cmd).await;
                    ui_tx.send(UIEvent::CommandDone(cmd.uuid())).unwrap();

                    if let Err(error) = result {
                        tracing::error!(?error, %cmd, "Failed to handle command {cmd} with error {error:#}");
                        ui_tx
                            .send(
                                ChatMessage::new_system(format!(
                                    "Failed to handle command: {error:#}"
                                ))
                                .uuid(cmd.uuid())
                                .into(),
                            )
                            .unwrap();
                    };
                });
            }

            tracing::warn!("CommandHandler shutting down");
        })
    }

    #[tracing::instrument(skip_all, fields(otel.name = %cmd.to_string(), uuid = %cmd.uuid()), err)]
    async fn handle_command(
        &self,
        repository: &Repository,
        ui_tx: &mpsc::UnboundedSender<UIEvent>,
        cmd: &Command,
    ) -> Result<()> {
        let now = std::time::Instant::now();
        tracing::warn!("Handling command {cmd}");

        // Integrating command handling logic for robustness and proper command delivery
        match cmd {
            Command::StopAgent { uuid } => {
                let mut locked_agents = self.agents.write().await;
                let Some(agent) = locked_agents.get_mut(uuid) else {
                    let _ = ui_tx.send(
                        ChatMessage::new_system("No agent found, ensure startup is complete.")
                            .uuid(*uuid)
                            .into(),
                    );
                    return Ok(());
                };

                let _ = ui_tx.send(
                    ChatMessage::new_system("Agent will stop after current completion")
                        .uuid(*uuid)
                        .into(),
                );

                // Handle agent stopping commands effectively
                agent.stop().await;
            }
            Command::IndexRepository { .. } => {
                let (command_responder, _guard) = self.spawn_command_responder(&cmd.uuid());
                indexing::index_repository(repository, Some(command_responder)).await?;
            }
            Command::ShowConfig { uuid } => {
                ui_tx
                    .send(
                        ChatMessage::new_system(toml::to_string_pretty(repository.config())?)
                            .uuid(*uuid)
                            .into(),
                    )
                    .unwrap();
            }
            Command::Chat { uuid, ref message } => {
                let agent = self.find_or_start_agent_by_uuid(*uuid, message).await?;
                agent.query(message).await?;
            }
            Command::Quit { .. } => unreachable!("Quit should be handled earlier"),
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
        let elapsed = now.elapsed();
        ui_tx
            .send(
                ChatMessage::new_system(format!(
                    "Command {cmd} finished in {:.2} seconds",
                    elapsed.as_secs_f64().round()
                ))
                .uuid(cmd.uuid())
                .into(),
            )
            .unwrap();

        Ok(())
    }

    async fn find_or_start_agent_by_uuid(&self, uuid: Uuid, query: &str) -> Result<RunningAgent> {
        if let Some(agent) = self.agents.read().await.get(&uuid) {
            return Ok(agent.clone());
        }

        let (responder, handle) = self.spawn_command_responder(&uuid);

        let agent = agent::build_agent(uuid, &self.repository.to_string(), query, responder)?;
        let running_agent = RunningAgent {
            agent: Arc::new(Mutex::new(agent)),
            handle: Arc::new(handle),
        };

        self.agents.write().await.insert(uuid, running_agent.clone());

        Ok(running_agent)
    }

    fn spawn_command_responder(&self, uuid: &Uuid) -> (CommandResponder, JoinHandle<()>) {
        let mut command_responder = CommandResponder::default().with_uuid(*uuid);

        let ui_tx_clone = self.ui_tx.clone().expect("UI transmitter expected");

        let cloned_responder = command_responder.clone();
        let handle = task::spawn(async move {
            while let Some(response) = command_responder.recv().await {
                match response {
                    CommandResponse::Chat(msg) => {
                        let _ = ui_tx_clone.send(msg.into());
                    }
                    CommandResponse::ActivityUpdate(uuid, state) => {
                        let _ = ui_tx_clone.send(UIEvent::ActivityUpdate(uuid, state));
                    }
                }
            }
        });

        (cloned_responder, handle)
    }
}
