use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Wrap};

use crate::frontend::App;

use super::message_formatting::format_chat_message;

pub struct ChatMessagesWidget;

impl ChatMessagesWidget {
    pub fn render(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
        let num_chats = app.chats.len();
        let Some(current_chat) = app.current_chat_mut() else {
            return;
        };
        let mut messages = current_chat.messages.clone();

        if messages.is_empty() && num_chats == 1 {
            messages.push(crate::chat_message::ChatMessage::new_system(
                "Let's get kwekking. Start chatting with an agent and confirm with ^s to send! At any time you can type `/help` to list keybindings and other slash commands.",
            ));
        }
        let chat_content: Text = messages
            .iter()
            .flat_map(|m| format_chat_message(current_chat, m))
            .collect();

        // Since we are rendering the chat, we can reset the new message count
        current_chat.new_message_count = 0;

        // Unify borders
        let border_set = symbols::border::Set {
            top_right: symbols::line::NORMAL.horizontal_down,
            ..symbols::border::PLAIN
        };

        let message_block = Block::default()
            .border_set(border_set)
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .padding(Padding::horizontal(1));

        let chat_messages = Paragraph::new(chat_content)
            .block(message_block)
            .wrap(Wrap { trim: false });

        // We need to consider the available area height to calculate how much can be shown
        //
        // Because the paragraph waps the text, we need to calculate the number of lines
        // from the paragraph directly.
        // Calculate number of lines based on messages (or equivalent content)
        let message_count = messages.len();
        current_chat.vertical_scroll_state = current_chat
            .vertical_scroll_state
            .content_length(message_count);

        // Max scroll to halfway view-height of last content
        if current_chat.vertical_scroll >= message_count.saturating_sub(1) {
            // Auto-scroll to the bottom if tailing is enabled
            if current_chat.is_tail_enabled {
                current_chat.vertical_scroll = message_count.saturating_sub(1);
            }
        // Ensure proper closure of all code blocks within the render function
        // Review indentation and assure all bracketing is balanced
    }

        #[allow(clippy::cast_possible_truncation)]
        let chat_messages = chat_messages.scroll((current_chat.vertical_scroll as u16, 0));

        f.render_widget(chat_messages, area);

        // Render scrollbar
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑")) // Fixed the unterminated string
                .end_symbol(Some("↓")),
            area,
            &mut current_chat.vertical_scroll_state,
        );
    }
}
