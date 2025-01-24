#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::ToolSummarizer;
    use crate::test_utils::test_repository;
    use async_trait::async_trait;
    use std::sync::Arc;
    use swiftide::chat_completion::ToolOutput;
    use swiftide::traits::{SimplePrompt, Tool, ToolSpec};

    // Mock implementation of SimplePrompt
    struct MockPrompt {}

    #[async_trait]
    impl SimplePrompt for MockPrompt {
        async fn prompt(
            &self,
            _prompt: swiftide::prompt::Prompt,
        ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok("mocked summary".into())
        }
    }

    // Mock implementation of Tool
    struct MockTool {
        name: String,
    }

    #[async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            &self.name
        }

        fn tool_spec(&self) -> ToolSpec {
            ToolSpec {
                description: "A mock tool".into(),
                ..Default::default()
            }
        }

        async fn call(&self, _args: &str) -> Result<ToolOutput, String> {
            Ok(ToolOutput::new("mocked tool output"))
        }
    }

    #[tokio::test]
    async fn test_summarize_hook() {
        let repo = test_repository();
        let llm = Arc::new(MockPrompt {});
        let tools_to_summarize = vec!["mock_tool"];
        let available_tools = vec![Box::new(MockTool {
            name: "mock_tool".into(),
        }) as Box<dyn Tool>];

        let tool_summarizer =
            ToolSummarizer::new(llm, &tools_to_summarize, &available_tools, "123abc");

        // Here we would need to simulate a tool call and tool output, pseudo code below:
        // let context = ...;
        // let tool_call = ...;
        // let tool_output = ...;
        // let hook = tool_summarizer.summarize_hook();
        // assert_eq!(hook(context, tool_call, tool_output).await.is_ok(), true);

        // Note: Actual context, tool_call, and tool_output instances need to be created.
        // This is a placeholder to show where to start.
    }
}
