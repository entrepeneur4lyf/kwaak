use anyhow::Result;
use std::borrow::{Borrow, Cow};
use std::io;
use std::time::Duration;

use ratatui::Terminal;

use crossterm::event::{self, KeyCode, KeyEvent};

use tokio::sync::mpsc;
use tokio::task;

use crate::commands::Command;
use crate::ui;

// Refresh rate in milliseconds
const REFRESH_RATE: u64 = 10;

pub struct App {
    pub input: String,
    pub messages: Vec<Message>,
    pub ui_tx: mpsc::UnboundedSender<UIEvent>,
    pub should_quit: bool,
}

pub enum Message {
    User(String),
    System(String),
    Command(Command),
}

impl Message {
    pub fn new_user(msg: impl Into<String>) -> Message {
        Message::User(msg.into())
    }

    pub fn new_system(msg: impl Into<String>) -> Message {
        Message::System(msg.into())
    }

    pub fn new_command(cmd: impl Into<Command>) -> Message {
        Message::Command(cmd.into())
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::User(msg) => write!(f, "You: {}", msg),
            Message::System(msg) => write!(f, "System: {}", msg),
            Message::Command(cmd) => write!(f, "Command: {}", cmd),
        }
    }
}

impl App {
    pub fn from_ui_tx(ui_tx: mpsc::UnboundedSender<UIEvent>) -> App {
        App {
            input: String::new(),
            messages: Vec::new(),
            ui_tx,
            should_quit: false,
        }
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    if self.input.starts_with("/") {
                        if let Ok(cmd) = Command::parse(&self.input) {
                            self.messages.push(Message::new_command(cmd.clone()));
                            let _ = self.ui_tx.send(UIEvent::Command(cmd));
                        } else {
                            self.messages.push(Message::new_system("Unknown command"));
                        }
                    } else {
                        self.messages.push(Message::new_user(&self.input));
                    }

                    self.input.clear();
                }
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}

// Event handling
pub enum UIEvent {
    Input(KeyEvent),
    Tick,
    Command(Command),
}

pub async fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let (ui_tx, mut ui_rx) = mpsc::unbounded_channel();
    let mut app = App::from_ui_tx(ui_tx.clone());

    // Spawn a blocking task to read input events
    let ui_tx_clone = ui_tx.clone();
    task::spawn_blocking(move || {
        loop {
            // Poll for input events
            if event::poll(Duration::from_millis(REFRESH_RATE * 10)).unwrap() {
                if let crossterm::event::Event::Key(key) = event::read().unwrap() {
                    let _ = ui_tx_clone.send(UIEvent::Input(key));
                }
            }

            // Send a tick event, ignore if the receiver is gone
            let _ = ui_tx_clone.send(UIEvent::Tick);
            // Sleep for the tick rate
            std::thread::sleep(Duration::from_millis(REFRESH_RATE));
        }
    });

    loop {
        // Draw the UI
        terminal.draw(|f| ui::ui(f, &app))?;

        // Handle events
        if let Some(event) = ui_rx.recv().await {
            match event {
                UIEvent::Input(key) => {
                    app.on_key(key);
                }
                UIEvent::Tick => {
                    // Handle periodic tasks if necessary
                }
                UIEvent::Command(cmd) => match cmd {
                    Command::Quit => {
                        app.should_quit = true;
                    }
                },
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
