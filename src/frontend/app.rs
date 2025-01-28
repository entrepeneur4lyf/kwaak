use std::sync::mpsc;
use ratatui::Terminal;
use ratatui::backend::{Backend, TermionBackend};
use ratatui::widgets::{Block, Borders};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crate::chat::{Chat, ChatState};
use crate::error::Result;
use crate::store::Store;
use crate::event::{UIEvent, Command};
use crate::chat_mode::{on_key, ChatMessagesWidget};
use crate::frontend::app::{App, AppMode};
use termion::event::Key;
use tui::widgets::Tabs;
use std::io;

impl App {
    pub fn run(&mut self) -> Result<()> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;
        terminal.enter_alternate_screen()?;
        crossterm::execute!(io::stdout(), EnableMouseCapture)?;

        let (tx, rx) = mpsc::channel();
        self.ui_tx = tx.clone();
        self.ui_rx = Some(rx);

        loop {
            terminal.draw(|f| self.draw(f))?;

            if event::poll(std::time::Duration::from_millis(200))? {
                if let Event::Key(key_event) = event::read()? {
                    if let KeyCode::Char('q') = key_event.code {
                        break;
                    }
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }

            self.recv_messages().await;

            if self.mode == AppMode::Quit {
                break;
            }
        }

        terminal.leave_alternate_screen()?;
        crossterm::execute!(io::stdout(), DisableMouseCapture)?;
        terminal.show_cursor()?;

        Ok(())
    }
}
