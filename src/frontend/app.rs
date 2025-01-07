use anyhow::Result;
use std::time::Duration;
use strum::IntoEnumIterator as _;
use tui_logger::TuiWidgetState;
use tui_textarea::TextArea;
use uuid::Uuid;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, ListState, Padding, Paragraph, Tabs},
};

use crossterm::event::{self, KeyCode, KeyEvent};

use tokio::sync::mpsc;
use tokio::task;

use crate::{
    chat::{Chat, ChatState},
    chat_message::ChatMessage,
    commands::Command,
    frontend,
};

use super::{chat_mode, logs_mode, UIEvent, UserInputCommand};

const TICK_RATE: u64 = 250;
const HEADER: &str = include_str!("ascii_logo");

/// Handles user and TUI interaction
pub struct App<'a> {
    // /// The chat input
    // pub input: String,
    pub text_input: TextArea<'a>,

    /// All known chats
    pub chats: Vec<Chat>,

    /// UUID of the current chat
    pub current_chat: uuid::Uuid,

    /// Holds the sender of UI events for later cloning if needed
    pub ui_tx: mpsc::UnboundedSender<UIEvent>,

    /// Receives UI events (key presses, commands, etc)
    pub ui_rx: mpsc::UnboundedReceiver<UIEvent>,

    /// Sends commands to the backend
    pub command_tx: Option<mpsc::UnboundedSender<Command>>,

    /// Mode the app is in, manages the which layout is rendered and if it should quit
    pub mode: AppMode,

    /// Tracks the current selected state in the UI
    pub chats_state: ListState,

    /// Tab names
    pub tab_names: Vec<&'static str>,

    /// Index of selected tab
    pub selected_tab: usize,

    /// States when viewing logs
    pub log_state: TuiWidgetState,

    /// Commands that relate to boot, and not a chat
    pub boot_uuid: Uuid,

    /// Skip indexing on boot
    pub skip_indexing: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AppMode {
    #[default]
    Chat,
    Logs,
    Quit,
}

impl AppMode {
    fn on_key(self, app: &mut App, key: KeyEvent) {
        match self {
            AppMode::Chat => chat_mode::on_key(app, key),
            AppMode::Logs => logs_mode::on_key(app, key),
            AppMode::Quit => (),
        }
    }

    fn ui(self, f: &mut ratatui::Frame, area: Rect, app: &mut App) {
        match self {
            AppMode::Chat => chat_mode::ui(f, area, app),
            AppMode::Logs => logs_mode::ui(f, area, app),
            AppMode::Quit => (),
        }
    }

    fn tab_index(self) -> Option<usize> {
        match self {
            AppMode::Chat => Some(0),
            AppMode::Logs => Some(1),
            AppMode::Quit => None,
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(AppMode::Chat),
            1 => Some(AppMode::Logs),
            _ => None,
        }
    }
}

impl Default for App<'_> {
    fn default() -> Self {
        let (ui_tx, ui_rx) = mpsc::unbounded_channel();

        let chat = Chat {
            name: "Chat #1".to_string(),
            ..Chat::default()
        };

        Self {
            skip_indexing: false,
            text_input: new_text_area(),
            current_chat: chat.uuid,
            chats: vec![chat],
            ui_tx,
            ui_rx,
            command_tx: None,
            mode: AppMode::default(),
            chats_state: ListState::default().with_selected(Some(0)),
            tab_names: vec!["[F1] Chats", "[F2] Logs"],
            log_state: TuiWidgetState::new()
                .set_default_display_level(log::LevelFilter::Off)
                .set_level_for_target("kwaak", log::LevelFilter::Info)
                .set_level_for_target("swiftide", log::LevelFilter::Info),
            selected_tab: 0,
            boot_uuid: Uuid::new_v4(),
        }
    }
}

fn new_text_area() -> TextArea<'static> {
    let mut text_area = TextArea::default();

    text_area.set_placeholder_text("Send a message to an agent ...");
    text_area.set_placeholder_style(Style::default().fg(Color::Gray));
    text_area.set_cursor_line_style(Style::reset());

    text_area
}

impl App<'_> {
    async fn recv_messages(&mut self) -> Option<UIEvent> {
        self.ui_rx.recv().await
    }

    #[allow(clippy::unused_self)]
    pub fn supported_commands(&self) -> Vec<UserInputCommand> {
        UserInputCommand::iter().collect()
    }

    pub fn send_ui_event(&self, msg: impl Into<UIEvent>) {
        let event = msg.into();
        tracing::debug!("Sending ui event {event}");
        if let Err(err) = self.ui_tx.send(event) {
            tracing::error!("Failed to send ui event {err}");
        }
    }

    pub fn reset_text_input(&mut self) {
        self.text_input = new_text_area();
    }

    fn on_key(&mut self, key: KeyEvent) {
        // Always quit on ctrl q
        if key.modifiers == crossterm::event::KeyModifiers::CONTROL
            && key.code == KeyCode::Char('q')
        {
            tracing::warn!("Ctrl-Q pressed, quitting");
            return self.send_ui_event(UIEvent::Quit);
        }

        if let KeyCode::F(index) = key.code {
            let index = index - 1;
            if let Some(mode) = AppMode::from_index(index as usize) {
                return self.change_mode(mode);
            }
        }

        self.mode.on_key(self, key);
    }

    #[tracing::instrument(skip(self))]
    pub fn dispatch_command(&mut self, cmd: &Command) {
        if let Some(chat) = self.current_chat_mut() {
            chat.transition(ChatState::Loading);
        }

        self.command_tx
            .as_ref()
            .expect("Command tx not set")
            .send(cmd.clone())
            .expect("Failed to dispatch command");
    }

    fn add_chat_message(&mut self, message: ChatMessage) {
        if message.uuid() == Some(self.boot_uuid) {
            return;
        }
        if let Some(chat) = self.find_chat_mut(message.uuid().unwrap_or(self.current_chat)) {
            chat.add_message(message);
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<()> {
        let handle = task::spawn(poll_ui_events(self.ui_tx.clone()));

        let mut has_indexed_on_boot = false;
        let mut splash = frontend::splash::Splash::default();

        if self.skip_indexing {
            has_indexed_on_boot = true;
        } else {
            self.dispatch_command(&Command::IndexRepository {
                uuid: self.boot_uuid,
            });
        }

        loop {
            // Draw the UI
            terminal.draw(|f| {
                if has_indexed_on_boot && splash.is_rendered() {
                    let base_area = self.draw_base_ui(f);

                    self.mode.ui(f, base_area, self);
                } else {
                    splash.render(f);
                }
            })?;
            if !splash.is_rendered() {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            if self.mode == AppMode::Quit {
                break;
            }

            // Handle events
            if let Some(event) = self.recv_messages().await {
                if !matches!(event, UIEvent::Tick | UIEvent::Input(_)) {
                    tracing::debug!("Received ui event: {:?}
