use insta::assert_snapshot;
use kwaak::chat::Chat;
use kwaak::chat_message::ChatMessageBuilder;
use kwaak::frontend::app::App;
use ratatui::backend::TestBackend;
use ratatui::{Terminal, TerminalOptions};

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
            let size = f.area();
            // Assuming correct UI function is defined
            kwaak::frontend::chat_mode::ui::ui(f, size, &mut app);
        })
        .unwrap();

    // Capture the rendered UI
    let rendered_ui = terminal.backend().to_string();

    // Assert snapshot
    assert_snapshot!(rendered_ui);
}

impl App<'_> {
    pub fn add_chat(&mut self, name: &str, messages: Vec<&str>) {
        let chat = Chat {
            name: name.to_string(),
            messages: messages
                .into_iter()
                .map(|msg| {
                    ChatMessageBuilder::default()
                        .content(msg.to_string())
                        .build()
                        .expect("Failed to build chat message")
                })
                .collect(),
            ..Default::default()
        };
        self.chats.push(chat);
    }
}
