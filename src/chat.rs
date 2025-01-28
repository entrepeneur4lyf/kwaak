use crate::chat::{ChatState, Message};
use crate::error::Result;
use crate::store::Store;
use crate::event::{UIEvent, Command};
use crate::chat_mode::{on_key, message_formatting};
use crate::frontend::app::{App, AppMode};
use termion::event::Key;
use tui::widgets::Tabs;

#[derive(Clone, Default, PartialEq)]
pub struct Chat {
    pub id: String,
    pub messages: Vec<Message>,
    pub auto_tailing_enabled: bool,
    pub vertical_scroll: usize,
    pub num_lines: usize,
    pub vertical_scroll_state: ScrollbarState,
    // Ensure required imports
    
    pub state: ChatState,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            id: String::new(),
            messages: Vec::new(),
            auto_tailing_enabled: true,
            vertical_scroll: 0,
            num_lines: 0,
            vertical_scroll_state: ScrollbarState::default(),
            state: ChatState::Ready,
        }
    }
}
