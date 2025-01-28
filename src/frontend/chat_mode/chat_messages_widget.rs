use super::common::format_chat_message;
use tui::widgets::{Block, Borders, StatefulWidget};
use tui::text::Text;
use crate::chat::Chat;
use crate::frontend::app::App;
use tui::layout::Rect;
use tui::Frame;
use tui::widgets::Scrollbar;
use tui::widgets::ScrollbarOrientation;

/// ChatMessagesWidget represents the rendering component
/// for displaying chat messages including auto-tailing functionality.
pub struct ChatMessagesWidget;

impl ChatMessagesWidget {
    pub fn render<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
        if let Some(current_chat) = app.current_chat() {
            let messages = current_chat.messages.iter().map(format_chat_message).collect::<Vec<_>>();
            let chat_messages = messages.join("\n");

            let auto_tailing_enabled = current_chat.auto_tailing_enabled &&
                current_chat.vertical_scroll >= current_chat.num_lines.saturating_sub(1);

            let chat_widget = Paragraph::new(chat_messages)
                .block(Block::default().borders(Borders::ALL).title("Chat"))
                .scroll((current_chat.vertical_scroll as u16, 0));

            f.render_widget(chat_widget, area);

            if auto_tailing_enabled {
                f.render_stateful_widget(
                    Scrollbar::new(ScrollbarOrientation::VerticalRight)
                        .begin_symbol(Some("|")),
                    area,
                    &mut current_chat.vertical_scroll_state,
                );
            }
        }
    }
}
