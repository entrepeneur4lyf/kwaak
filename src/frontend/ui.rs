use ratatui::prelude::*;
use ratatui::widgets::{Clear, Scrollbar, ScrollbarOrientation, Wrap};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

use crate::chat_message::ChatMessage;

use super::app::App;

pub fn ui(f: &mut ratatui::Frame, app: &mut App) {
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

fn render_chat_messages(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    f.render_widget(Clear, f.area());
    let chat_content: Text = app
        .messages
        .iter()
        .flat_map(|m| format_chat_message(m, area.width))
        .collect();

    let num_lines = chat_content.lines.len();

    app.vertical_scroll_state = app.vertical_scroll_state.content_length(num_lines);

    let chat_messages = Paragraph::new(chat_content)
        .block(Block::default().title("Chat").borders(Borders::ALL))
        .scroll((app.vertical_scroll, 0));

    f.render_widget(chat_messages, area);

    // Render scrollbar
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area,
        &mut app.vertical_scroll_state,
    );
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
    .wrap(Wrap { trim: true })
    .block(Block::default().title("Commands").borders(Borders::ALL));
    f.render_widget(commands, area);
}

fn format_chat_message(message: &ChatMessage, width: u16) -> Text {
    let (prefix, content) = match message {
        ChatMessage::User(msg) => ("You", msg.as_str()),
        ChatMessage::System(msg) => ("System", msg.as_str()),
        ChatMessage::Command(cmd) => ("Command", cmd.into()),
    };
    let prefix: Span = Span::styled(prefix, Style::default().fg(Color::Yellow));

    // skin.paragraph.align = termimad::Alignment::Unspecified;
    // skin.code_block.align = termimad::Alignment::Unspecified;
    // skin.limit_to_ascii();

    // skin.code_block.set_bg(crossterm::style::Color::Reset);
    // let text = skin
    //     .text(content, Some(width as usize - 4))
    //     .to_text()
    //     .to_string();
    // let text = rendered.to_text();
    // let content = Text::from(rendered.to_text().to_string());
    // let text = rendered.to_text().to_owned();

    let content: Text = tui_markdown::from_str(content);
    //
    Text::from(prefix) + content
}
