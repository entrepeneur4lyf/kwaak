use ratatui::prelude::*;
use ratatui::widgets::{
    HighlightSpacing, List, ListItem, Padding, Scrollbar, ScrollbarOrientation, Wrap,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

use crate::chat::{Chat, ChatState};
use crate::chat_message::ChatMessage;
use crate::frontend::App;

pub fn ui(f: &mut ratatui::Frame, area: Rect, app: &mut App) {
    // If we're rendering the current chat and it has new messages
    // set it as ready, clearing the new message
    if app.current_chat().has_new_messages() {
        app.current_chat_mut().set_ready();
    }

    // Create the main layout (vertical)
    let [main_area, input_area, help_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main area
            Constraint::Length(3), // User input bar (2 lines)
            Constraint::Length(3), // Commands display area
        ])
        .areas(area);

    // Split the main area into two columns
    let [chat_messages, chat_list] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80), // Left column (chat messages)
            Constraint::Percentage(20), // Right column (other info)
        ])
        .areas(main_area);

    // Render chat messages
    render_chat_messages(f, app, chat_messages);

    // Render other information
    render_chat_list(f, app, chat_list);

    // Render user input bar
    render_input_bar(f, app, input_area);

    // Render commands display area
    render_commands_display(f, app, help_area);
}

fn render_chat_messages(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let messages = app.current_chat().messages.clone();
    let chat_content: Text = messages.iter().flat_map(format_chat_message).collect();

    let num_lines = chat_content.lines.len();

    app.vertical_scroll_state = app.vertical_scroll_state.content_length(num_lines);

    let message_block = Block::default()
        .title("Chat")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));

    let chat_messages = Paragraph::new(chat_content)
        .block(message_block)
        .wrap(Wrap { trim: false })
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
fn render_chat_list(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let list: List = app
        .chats
        .iter()
        .map(format_chat_in_list)
        .collect::<List>()
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
        .block(Block::default().title("Chats").borders(Borders::ALL));

    f.render_stateful_widget(list, area, &mut app.chats_state);
}

fn format_chat_in_list(chat: &Chat) -> ListItem {
    let suffix = match chat.state {
        ChatState::Loading => " ...",
        ChatState::NewMessage => " *",
        ChatState::Ready => "",
    };

    ListItem::from(format!("{}{}", chat.name, suffix))
}

fn render_input_bar(f: &mut ratatui::Frame, app: &App, area: Rect) {
    if app.current_chat().is_loading() {
        let block = Block::default().title("Input").borders(Borders::ALL);
        let throbber = throbber_widgets_tui::Throbber::default().label("Kwaaking ...");

        f.render_widget(throbber, block.inner(area));
        return block.render(area, f.buffer_mut());
    }

    let block = Block::default().title("Input").borders(Borders::ALL);
    let input = Paragraph::new(app.input.as_str()).block(block);
    f.render_widget(input, area);
    // Set cursor position
    f.set_cursor_position(
        // Put cursor past the end of the input text
        #[allow(clippy::cast_possible_truncation)]
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

fn format_chat_message(message: &ChatMessage) -> Text {
    let prefix: Span = Span::styled(message.role().as_ref(), Style::default().fg(Color::Yellow));
    let content: Text = tui_markdown::from_str(message.content());

    Text::from(prefix) + content
}
