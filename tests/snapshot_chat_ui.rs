use crate::frontend::app::App;
use crate::frontend::chat_mode::ui;
use async_openai::Chat;
use insta::assert_snapshot;
use ratatui::backend::TestBackend;
use ratatui::prelude::*;
use ratatui::Frame;
use ratatui::Terminal;

#[test]
fn snapshot_test_chat_ui() {
    // Setup application state
    let mut app = App::default();
    app.add_chat("test_chat", vec!["Hello!", "How can I help?"]);

    // Render the UI
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::with_options(backend, TerminalOptions::default()).unwrap();
    terminal
        .draw(|f| {
            let size = f.size();
            ui::ui(f, size, &mut app);
        })
        .unwrap();

    // Capture the rendered UI
    let rendered_ui = format!("{:?}", terminal.backend_mut().as_ref());

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
