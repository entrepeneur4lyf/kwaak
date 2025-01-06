use crate::frontend::chat_mode::ui;
use crate::App;
use insta::assert_snapshot;
use ratatui::backend::TestBackend;
use ratatui::prelude::*;
use ratatui::terminal::{Terminal, TerminalOptions};
use ratatui::Frame;

#[test]
fn snapshot_test_chat_ui() {
    // Setup application state
    let mut app = App::default();
    app.add_chat("test_chat", vec!["Hello!", "How can I help?"]);

    // Render the UI
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new_with_options(backend, TerminalOptions::default()).unwrap();
    terminal
        .draw(|f| {
            let size = f.size();
            ui::ui(f, size, &mut app);
        })
        .unwrap();

    // Capture the rendered UI
    let mut frame = Frame::default();
    ui::ui(&mut frame, terminal.size().unwrap(), &mut app);
    let rendered_ui = format!("{:?}", frame);

    // Assert snapshot
    assert_snapshot!(rendered_ui);
}

impl App {
    fn add_chat(&mut self, name: &str, messages: Vec<&str>) {
        let chat = Chat {
            name: name.to_string(),
            messages: messages.into_iter().map(ToString::to_string).collect(),
            ..Default::default()
        };
        self.chats.push(chat);
    }
}
