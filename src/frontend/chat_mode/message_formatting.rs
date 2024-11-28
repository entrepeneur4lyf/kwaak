use ratatui::prelude::*;

use crate::{
    chat::Chat,
    chat_message::{ChatMessage, ChatRole},
};

mod message_styles {
    use super::{Color, Modifier, Style};

    pub const USER: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::ITALIC);

    pub const ASSISTANT: Style = Style::new()
        .fg(Color::Rgb(200, 160, 255))
        .add_modifier(Modifier::BOLD);

    pub const SYSTEM: Style = Style::new().fg(Color::DarkGray).add_modifier(Modifier::DIM);

    pub const TOOL_DONE: Style = Style::new().fg(Color::Green).add_modifier(Modifier::DIM);

    pub const TOOL_CALLED: Style = Style::new().fg(Color::DarkGray).add_modifier(Modifier::DIM);

    pub const COMMAND: Style = Style::new()
        .fg(Color::LightMagenta)
        .add_modifier(Modifier::BOLD);
}
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn get_style_and_prefix(role: &ChatRole) -> (&'static str, Style) {
    match role {
        ChatRole::User => ("▶ ", message_styles::USER),
        ChatRole::Assistant => ("✦ ", message_styles::ASSISTANT),
        ChatRole::System => ("ℹ ", message_styles::SYSTEM),
        ChatRole::Tool => ("⚙ ", message_styles::TOOL_DONE), // unused
        ChatRole::Command => ("» ", message_styles::COMMAND),
    }
}

// TODO: Maybe have tool state just on the message?
pub fn format_chat_message<'a>(current_chat: &Chat, message: &'a ChatMessage) -> Text<'a> {
    let (prefix, style) = get_style_and_prefix(message.role());

    // Render markdown first
    let mut rendered_text = tui_markdown::from_str(message.content());

    // Prepend the styled prefix to the first line
    if let Some(first_line) = rendered_text.lines.first_mut() {
        first_line.spans.insert(0, Span::styled(prefix, style));
    }

    // Remove background color from all lines
    // tui_markdown adds background colors and underlines to some headers
    rendered_text.lines.iter_mut().for_each(|line| {
        line.spans.iter_mut().for_each(|span| {
            span.style = span
                .style
                .bg(Color::Reset)
                .remove_modifier(Modifier::UNDERLINED);
        });
    });

    if let Some(swiftide::chat_completion::ChatMessage::Assistant(.., Some(tool_calls))) =
        message.original()
    {
        if !message.content().is_empty() {
            rendered_text.push_line(Line::from("\n\n"));
        }
        for tool_call in tool_calls {
            let is_done = current_chat.is_tool_call_completed(tool_call.id());
            let tool_call_text = format_tool_call(tool_call);
            let tool_prefix = "⚙ ";

            if is_done {
                // add a suffix checkmark on the end
                let checkmark = " ✓";
                rendered_text.lines.push(Line::styled(
                    [tool_prefix, &tool_call_text, checkmark].join(" "),
                    message_styles::TOOL_DONE,
                ));
            } else {
                rendered_text.lines.push(Line::styled(
                    [tool_prefix, &tool_call_text].join(" "),
                    message_styles::TOOL_CALLED,
                ));
            }
        }
    }

    if !message.role().is_assistant() {
        for line in &mut rendered_text.lines {
            for span in &mut line.spans {
                span.style = style;
            }
        }
    }

    rendered_text
}

fn format_tool_call(tool_call: &swiftide::chat_completion::ToolCall) -> String {
    // If args, parse them as a json value, then if its just one, render only the value, otherwise
    // limit the output to 20 characters
    let mut formatted_args = tool_call.args().map_or("no arguments".to_string(), |args| {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(args) {
            if let Some(obj) = parsed.as_object() {
                if obj.keys().count() == 1 {
                    let key = obj.keys().next().unwrap();
                    let val = obj[key].as_str().unwrap_or_default();

                    return val.to_string();
                }

                return args.to_string();
            }

            "no_arguments".to_string()
        } else {
            "no_arguments".to_string()
        }
    });

    if formatted_args.len() > 20 {
        formatted_args.truncate(20);
        formatted_args.push_str("...");
    }

    format!(
        "calling tool `{}` with `{}`",
        tool_call.name(),
        formatted_args
    )
}
