use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::frontend::App;

pub struct HelpSectionWidget;

impl HelpSectionWidget {
    pub fn render(f: &mut ratatui::Frame, app: &App, area: Rect) {
        let border_set = symbols::border::Set {
            top_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::PLAIN
        let num_commands = app.supported_commands().len();
        let num_cols = 2; // Set to 2 columns for better visibility
        let commands_per_col = (num_commands as f32 / num_cols as f32).ceil() as usize;

        let mut columns = Vec::new();
        for col_num in 0..num_cols {
            let col_commands = &app.supported_commands()[col_num * commands_per_col..
        for column_line in column_lines {
            Paragraph::new(column_line)
                .block(
                    Block::default()
                        .title("Chat commands".bold())
                        .title_alignment(Alignment::Center)
                        .borders(Borders::TOP | Borders::RIGHT)
                        .border_set(border_set)
                        .padding(Padding::uniform(1)),
                )
                .render(top, f.buffer_mut());
        }
                .border_set(border_set)
                .padding(Padding::uniform(1)),
        )
        .render(top, f.buffer_mut());

        Paragraph::new(
            [
                "Page Up/Down - Scroll",
                "End - Scroll to end",
                "^s - Send message",
                "^x - Stop agent",
                "^n - New chat",
                "^q - Quit",
            ]
            .iter()
            .map(|h| Line::from(h.bold()))
            .collect::<Vec<Line>>(),
        )
        .block(
            Block::default()
                .title("Keybindings".bold())
                .title_alignment(Alignment::Center)
                .border_set(border_set)
                .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
                .padding(Padding::uniform(1)),
        )
        .render(bottom, f.buffer_mut());
    }
}
