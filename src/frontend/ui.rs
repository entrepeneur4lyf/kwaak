use ratatui::prelude::*;
use ratatui::widgets::Wrap;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

use crate::chat_message::ChatMessage;

use super::app::App;

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

    // Render chat messages
    render_chat_messages(f, app, main_chunks[0]);

    // Render other information
    render_other_info(f, main_chunks[1]);

    // Render user input bar
    render_input_bar(f, app, chunks[1]);

    // Render commands display area
    render_commands_display(f, app, chunks[2]);
}

fn render_chat_messages(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let messages: Vec<Text> = app
        .messages
        .iter()
        .map(|m| format_chat_message(m))
        .collect();

    let chat_content = messages
        .into_iter()
        .fold(Text::default(), |acc, msg| acc + msg);

    let chat_messages = Paragraph::new(chat_content)
        .block(Block::default().title("Chat").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    f.render_widget(chat_messages, area);
}

fn render_other_info(f: &mut ratatui::Frame, area: Rect) {
    let other_info =
        Paragraph::new("Other info").block(Block::default().title("Info").borders(Borders::ALL));
    f.render_widget(other_info, area);
}

fn render_input_bar(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Input").borders(Borders::ALL));
    f.render_widget(input, area);
    // Set cursor position
    f.set_cursor_position(
        // Put cursor past the end of the input text
        (area.x + app.input.len() as u16 + 1, area.y + 1),
    );
}

fn render_commands_display(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let commands = Paragraph::new(
        app.supported_commands()
            .iter()
            .map(|c| format!("/{c}"))
            .collect::<Vec<_>>()
            .join(" "),
    )
    .block(Block::default().title("Commands").borders(Borders::ALL));
    f.render_widget(commands, area);
}

fn format_chat_message(message: &ChatMessage) -> Text {
    let (prefix, content) = match message {
        ChatMessage::User(msg) => ("You", msg.as_str()),
        ChatMessage::System(msg) => ("System", msg.as_str()),
        ChatMessage::Command(cmd) => ("Command", cmd.into()),
    };
    let prefix: Span = Span::styled(prefix, Style::default().fg(Color::Yellow));
    let content: Text = tui_markdown::from_str(content);

    Text::from(prefix) + content
}
