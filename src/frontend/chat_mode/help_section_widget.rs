use crate::frontend::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph},
};

pub struct HelpSectionWidget;
let border_set = symbols::border::Set {
    top_right: symbols::line::NORMAL.vertical_left,
    top_left: symbols::line::NORMAL.vertical_right,
    bottom_right: symbols::line::NORMAL.vertical_left,
let top_bottom = Layout::vertical([
    Constraint::Length((app.supported_commands().len() / 2) as u16 + 3),
    Constraint::Min(6),
])
.split(area);
let top = top_bottom[0];
let bottom = top_bottom[1];
    vertical_left: symbols::line::NORMAL.vertical,
    vertical_right: symbols::line::NORMAL.vertical,
    horizontal_top: symbols::line::NORMAL.horizontal,
    horizontal_bottom: symbols::line::NORMAL.horizontal_top,
    horizontal: symbols::line::NORMAL.horizontal,
};
let rects = Layout::vertical([
    Constraint::Length((app.supported_commands().len() / 2) as u16 + 3),
    Constraint::Min(6),
])
.split(area);
let [top, bottom] = [rects[0], rects[1]];
            ..symbols::border::PLAIN
        };
        let [top, bottom] = Layout::vertical([
            Constraint::Length((app.supported_commands().len() / 2) as u16 + 3), // Assuming two columns
            Constraint::Min(6), // Adjusting to ensure keybindings are more visible
        ])
        .split(area);

        let commands = app
            .supported_commands()
            .chunks(app.supported_commands().len() / 2 + 1)
            .zip([Alignment::Left, Alignment::Right].iter())
            .map(|(chunk, &alignment)| {
                Paragraph::new(
                    chunk
                        .iter()
                        .map(|c| Line::from(format!("/{c}").bold()))
                        .collect::<Vec<Line>>(),
                )
                .block(
                    Block::default()
                        .title("Chat commands".bold())
                        .title_alignment(Alignment::Center)
                        .borders(Borders::TOP | Borders::RIGHT)
                        .border_set(border_set)
                        .padding(Padding::uniform(1)),
                )
                .alignment(alignment)
            });

        for (i, paragraph) in commands.enumerate() {
            let chunk_area =
                Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(top)[i];
            paragraph.render(chunk_area, f.buffer_mut());
        }

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
