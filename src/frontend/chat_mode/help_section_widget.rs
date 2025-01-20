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
            let start_index = col_num * commands_per_col;
            let end_index = ((col_num + 1) * commands_per_col).min(num_commands);

            let col_commands = &app.supported_commands()[start_index..end_index];
            columns.push(col_commands
                .iter()
                .map(|c| Line::from(format!("/{c} ").bold()))
                .collect::<Vec<Line>>());
        }

        let column_lines = (0..commands_per_col).map(|i| {
            columns.iter().filter_map(|col| col.get(i)).cloned().collect::<Vec<Line>>()
        }).collect::<Vec<Vec<Line>>>();

        let mut rendered_lines = Vec::new();
        for column_line in column_lines {
            for line in column_line {
                rendered_lines.push(line);
            }
        }

        Paragraph::new(rendered_lines)
            .block(
                Block::default()
                    .title("Chat commands".bold())
                    .title_alignment(Alignment::Center)
                    .borders(Borders::TOP | Borders::RIGHT)
                    .border_set(border_set)
                    .padding(Padding::uniform(1)),
            )
            .render(top, f.buffer_mut());
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
