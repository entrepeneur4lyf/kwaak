use ratatui::{Frame, widgets::{Block, Borders, Paragraph}, layout::Rect};
use super::App;

pub struct ChatMessagesWidget;

impl ChatMessagesWidget {
    pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
        // Check if there are chat messages
        if let Some(current_chat) = app.chats.get_mut(&app.current_chat_id) {
            // Prepare messages for rendering
            let messages: Vec<String> = current_chat
                .messages
                .iter()
                .map(|message| format_chat_message(message))
                .collect();

            // Add default system message if no messages exist
            if messages.is_empty() && app.chats.len() == 1 {
                messages.push("[System] Start chatting...".to_string());
            }

            // Format messages into a paragraph
            let paragraph = Paragraph::new(messages.join("\n")).block(Block::default().borders(Borders::ALL));

            // Calculate scroll offset
            let scroll_offset = if current_chat.auto_tailing_enabled {
                messages.len().saturating_sub(area.height as usize)
            } else {
                current_chat.message_scroll_offset
            };

            // Render paragraph with scroll offset
            f.render_widget(paragraph, area.subarea(0, scroll_offset as u16));

            // Render scrollbar if necessary
            if messages.len() > area.height as usize {
                f.render_widget(
                    Scrollbar::default()
                        .highlight_symbol("▞")
                        .highlight_style(Style::default().fg(Color::Cyan)),
                    area,
                );
            }
        }
    }
}

fn format_chat_message(message: &str) -> String {
    format!("- {}", message)
}
