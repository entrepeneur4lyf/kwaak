use swiftide::traits::{CommandError, CommandOutput};

pub fn strip_markdown_tags(text: &str) -> String {
    if text.starts_with("```markdown") && text.ends_with("```") {
        text[12..text.len() - 3].trim().to_string()
    } else {
        text.to_string()
    }
}

// TODO: Would be nice if this was a method on a custom result in swiftide
pub fn accept_non_zero_exit(
    result: Result<CommandOutput, CommandError>,
) -> Result<CommandOutput, CommandError> {
    match result {
        Ok(output) | Err(CommandError::FailedWithOutput(output)) => Ok(output),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swiftide::traits::{CommandError, CommandOutput};

    #[test]
    fn test_strip_markdown_tags() {
        // Case: String with surrounding ```markdown tags
        let input = "```markdown\nThis is a test.\n```";
        let expected = "This is a test.";
        assert_eq!(strip_markdown_tags(input), expected);

        // Case: String without surrounding ```markdown tags
        let input = "This is a test.";
        let expected = "This is a test.";
        assert_eq!(strip_markdown_tags(input), expected);

        // Case: Empty string
        let input = "";
        let expected = "";
        assert_eq!(strip_markdown_tags(input), expected);

        // Case: String with only opening ```markdown tag
        let input = "```markdown\nThis is a test.";
        let expected = "```markdown\nThis is a test.";
        assert_eq!(strip_markdown_tags(input), expected);

        // Case: String with only closing ``` tag
        let input = "This is a test.\n```";
        let expected = "This is a test.\n```";
        assert_eq!(strip_markdown_tags(input), expected);

        // Case: Raw string
        let input = r"```markdown
        This is a test.
        ```";
        let expected = "This is a test.";
        assert_eq!(strip_markdown_tags(input), expected);
    }

    #[test]
    fn test_accept_non_zero_exit() {
        // Case: Result is Ok(CommandOutput)
        let output = CommandOutput::new(vec!["Output Line 1".into(), "Output Line 2".into()]);
        let result = Ok(output.clone());
        assert_eq!(accept_non_zero_exit(result), Ok(output));
        
        // Case: Result is Err(CommandError::FailedWithOutput(CommandOutput))
        let err_result = Err(CommandError::FailedWithOutput(output.clone()));
        assert_eq!(accept_non_zero_exit(err_result), Ok(output));

        // Case: Result is Err(CommandError::Other)
        let other_error = CommandError::Other("Unknown Error".into());
        let err_result = Err(other_error.clone());
        assert_eq!(accept_non_zero_exit(err_result), Err(other_error));
    }
}
