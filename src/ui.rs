use ratatui::prelude::*;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;
use crate::chat_message::ChatMessage;

pub fn ui(f: &mut ratatui::Frame, app: &App) {
    // Create the main layout (vertical)
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main area
            Constraint::Length(3), // User input bar (2 lines)
            Constraint::Length(3), // Commands display area
        ])
        .split(area);

    // Split the main area into two columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80), // Left column (chat messages)
            Constraint::Percentage(20), // Right column (other info)
        ])
        .split(chunks[0]);

    // Left column - Chat messages
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(format_chat_message(m)))
        .collect();

    let chat_messages =
        List::new(messages).block(Block::default().title("Chat").borders(Borders::ALL));

    f.render_widget(chat_messages, main_chunks[0]);

    // Right column - Other information
    let other_info =
        Paragraph::new("Other info").block(Block::default().title("Info").borders(Borders::ALL));
    f.render_widget(other_info, main_chunks[1]);

    // User input bar
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Input").borders(Borders::ALL));
    f.render_widget(input, chunks[1]);
    // Set cursor position
    f.set_cursor_position(
        // Put cursor past the end of the input text
        (chunks[1].x + app.input.len() as u16 + 1, chunks[1].y + 1),
    );

    // Commands display area
    let commands = Paragraph::new("/quit /show_config")
        .block(Block::default().title("Commands").borders(Borders::ALL));
    f.render_widget(commands, chunks[2]);
}

fn format_chat_message(message: &ChatMessage) -> Text {
    let (prefix, content) = match message {
        ChatMessage::User(msg) => ("You", msg.to_string()),
        ChatMessage::System(msg) => ("System", msg.to_string()),
        ChatMessage::Command(cmd) => ("Command", cmd.to_string()),
    };
    let prefix: Span = Span::styled(prefix, Style::default().fg(Color::Yellow));
    let content: Text = Text::from(content);

    Text::from(prefix) + content
}
