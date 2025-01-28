impl App {
    fn handle_ui_event(&mut self, event: UIEvent) {
        match event {
            UIEvent::ScrollUp => {
                let Some(current_chat) = self.current_chat_mut() else {
                    return;
                };
                current_chat.vertical_scroll = current_chat.vertical_scroll.saturating_sub(2);
                current_chat.vertical_scroll_state = current_chat
                    .vertical_scroll_state
                    .position(current_chat.vertical_scroll);
                current_chat.auto_tailing_enabled = false; // Disable auto-tailing on manual scroll
            }
            UIEvent::ScrollDown => {
                let Some(current_chat) = self.current_chat_mut() else {
                    return;
                };
                current_chat.vertical_scroll = current_chat.vertical_scroll.saturating_add(2);
                current_chat.vertical_scroll_state = current_chat
                    .vertical_scroll_state
                    .position(current_chat.vertical_scroll);
                current_chat.auto_tailing_enabled = false; // Disable auto-tailing on manual scroll
            }
            UIEvent::ScrollEnd => {
                let Some(current_chat) = self.current_chat_mut() else {
                    return;
                };
                // Keep the last 10 lines in view
                let scroll_position = current_chat.num_lines.saturating_sub(10);

                current_chat.vertical_scroll = scroll_position;
                current_chat.vertical_scroll_state =
                    current_chat.vertical_scroll_state.position(scroll_position);
                current_chat.auto_tailing_enabled = true; // Re-enable auto-tailing
            }
            // Other events...
            _ => {}
        }
    }

    // Other methods...
}
