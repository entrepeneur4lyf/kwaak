use crate::{
    chat_message::ChatMessage,
    commands::Command,
    frontend::{ui_event::UIEvent, ui_input_command::UserInputCommand, App},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn on_key(app: &mut App, key: &KeyEvent) {
    let mut current_input = app.text_input.lines().join("\n");

    match key.code {
        KeyCode::Enter => {
            // Simulates new line on Enter, sending the message
            if !current_input.is_empty() {
                let message = if current_input.starts_with('/') {
                    handle_input_command(app)
                } else {
                    app.dispatch_command(
                        app.current_chat_uuid,
                        Command::Chat {
                            message: current_input.clone(),
                        },
                    );

                    ChatMessage::new_user(current_input)
                };

                app.send_ui_event(UIEvent::ChatMessage(app.current_chat_uuid, message));
                app.reset_text_input();
            }
        }
        KeyCode::Backspace => {
            // Handle backspace to delete characters
            app.text_input.input(*key);
        }
        _ => {
            // Handle regular text input
            app.text_input.input(*key);
            current_input = app.text_input.lines().join("\n");

            // Check for manual line wrapping logic
            let input_width = 40; // Assume 40 as max chars per line for demo purposes
            if current_input.lines().last().unwrap_or("").len() >= input_width {
                app.text_input
                    .input(KeyEvent::new(KeyCode::Enter, event::KeyModifiers::NONE));
.input(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        }
    }
}

pub fn handle_input_command(app: &mut App) -> ChatMessage {
    let current_input = app.text_input.lines().join("\n");

    let Ok(cmd) = UserInputCommand::parse_from_input(&current_input) else {
        return ChatMessage::new_system("Unknown command").clone();
    };

    let message = ChatMessage::new_command(cmd.as_ref()).clone();

    app.send_ui_event(UIEvent::UserInputCommand(app.current_chat_uuid, cmd));

    message
}
