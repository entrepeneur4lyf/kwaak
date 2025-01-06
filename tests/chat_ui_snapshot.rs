use crate::frontend::chat_mode::ui;
use crate::frontend::App;
use insta::assert_snapshot;
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn test_chat_ui_snapshot() {
    // Initialize a test backend and terminal
    let backend = TestBackend::new(80, 24); // assuming a typical terminal size
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal test backend");

    // Set up the App state (mocked or default state)
    let mut app = App::new(); // You might need to adjust initialization

    // Set up a frame
    terminal
        .draw(|f| {
            let size = f.area();
            ui(f, size, &mut app);
        })
        .expect("Failed to draw into terminal");

    // Capture the buffer and convert to string for snapshot
    let buffer = terminal.backend().buffer().clone();

    // Convert buffer to a comparable format (e.g., String)
    let buffer_as_string = format!("{:?}", buffer);

    // Use insta to snapshot the buffer state
    assert_snapshot!("chat_ui_snapshot", buffer_as_string);
}
