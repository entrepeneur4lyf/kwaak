//! This module implements a conversation summarizer
//!
//! The summarizer counts number of completions in an agent, and if the count surpasses a set
//! value, the conversation is summarized and a summary is added to the conversation.
//!
//! Often when there are many messages, the context window can get too large for the agent to
//! effectively complete. This summarizer helps to keep the context window small and steers the
//! agent towards a more focused solution.
//!
//! The agent completes messages since the last summary.
use std::sync::{atomic::AtomicUsize, Arc};

use swiftide::{
    agents::hooks::AfterEachFn,
    chat_completion::{ChatCompletion, ChatMessage, Tool},
};
use futures::future::BoxFuture;
use tracing::Instrument as _;

const NUM_COMPLETIONS_FOR_SUMMARY: usize = 10;

#[derive(Clone)]
pub struct ConversationSummarizer {
    llm: Arc<dyn ChatCompletion>,
    available_tools: Vec<Box<dyn Tool>>,
    num_completions_since_summary: Arc<AtomicUsize>,
}

impl ConversationSummarizer {
    pub fn new(llm: Box<dyn ChatCompletion>, available_tools: &[Box<dyn Tool>]) -> Self {
        Self {
            llm: llm.into(),
            available_tools: available_tools.into(),
            num_completions_since_summary: Arc::new(0.into()),
        }
    }

    pub fn summarize_hook(self) -> impl AfterEachFn {
        move |context| {
            let llm = self.llm.clone();

            let prompt = self.prompt();

            let span = tracing::info_span!("summarize_conversation");

            let current_count = self
                .num_completions_since_summary
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

            if current_count < NUM_COMPLETIONS_FOR_SUMMARY {
                tracing::debug!(current_count, "Not enough completions for summary");

                return Box::pin(async move { Ok(()) });
            }

            self.num_completions_since_summary
                .store(0, std::sync::atomic::Ordering::SeqCst);

            Box::pin(
                async move {
                    let mut messages = filter_messages_since_summary(context.history().await);
                    messages.push(ChatMessage::new_user(prompt));

                    let summary = llm.complete(&messages.into()).await?;

                    if let Some(summary) = summary.message() {
                        tracing::debug!(summary = %summary, "Summarized tool output");
                        context.add_message(ChatMessage::new_summary(summary)).await;
                    } else {
                        tracing::error!("No summary generated, this is a bug");
                    }

                    Ok(())
                }
                .instrument(span),
            )
        }
    }

    fn prompt(&self) -> String {
        let available_tools = self
            .available_tools
            .iter()
            .map(|tool| format!("- **{}**: {}", tool.name(), tool.tool_spec().description))
            .collect::<Vec<String>>()
            .join("\n");

        indoc::formatdoc!(
            "
        # Goal
        Summarize the conversation up to this point
            
        ## Requirements
        * Only include the summary in your response and nothing else.
        * When mentioning files include the full path
        * Be very precise
        * If a previous solution did not work, include that in your response. If a reason was
            given, include that as well.
        * Include any previous summaries in your response
        * Be extra detailed on the last step taken
        * Provide clear instructions on how to proceed. If applicable, include the tools that
            should be used.
        * Identify the goal the user wanted to achieve and clearly restate it

        ## Available tools
        {available_tools}

        ## Format
        * Start your response with the following header '# Summary'
        * Phrase each bullet point as if talking about 'you'
        
        # Example format
        ```
        # Summary

        ## Previously you did
        * <concise summary of each step>
        * You tried to run the tests but they failed. Here is why <...>

        ## Since then you did
        * <Summary of steps since the last summary>

        ## Reflection
        <Concise reflection on the steps you took and why you took them>
        
        ## Suggested next steps
        1. <Suggested step>
        ```
       

        ",
            available_tools = available_tools
        )
    }
}

fn filter_messages_since_summary(messages: Vec<ChatMessage>) -> Vec<ChatMessage> {
    let mut summary_found = false;
    let mut messages = messages
        .into_iter()
        .rev()
        .filter(|m| {
            if summary_found {
                return matches!(m, ChatMessage::System(_));
            }
            if let ChatMessage::Summary(_) = m {
                summary_found = true;
            }
            true
        })
        .collect::<Vec<_>>();

    messages.reverse();

    messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Arc;
    use swiftide::chat_completion::{ChatCompletion, ChatMessage, CompletionResult, Tool};

    #[derive(Clone)]
    struct MockChatCompletion;

    #[async_trait]
    impl ChatCompletion for MockChatCompletion {
        async fn complete(&self, _: &[ChatMessage]) -> Result<CompletionResult, String> {
            Ok(CompletionResult::new(Some("Mock Summary".to_string()), None))
        }

        fn name(&self) -> &'static str {
            "mock"
        }
    }

    #[derive(Clone)]
    struct MockTool;

    impl Tool for MockTool {
        fn name(&self) -> &'static str {
            "mock_tool"
        }

        fn tool_spec(&self) -> Box<dyn swiftide::tool::ToolSpec> {
            Box::new(MockToolSpec {})
        }
    }

    struct MockToolSpec;

    impl swiftide::tool::ToolSpec for MockToolSpec {
        fn description(&self) -> &'static str {
            "Mock Tool Description"
        }
    }

    #[tokio::test]
    async fn test_summarize_hook() {
        let mock_llm = Box::new(MockChatCompletion);
        let mock_tool = Box::new(MockTool);
        let summarizer = ConversationSummarizer::new(mock_llm, &[mock_tool]);

        let mock_context = MockContext::new();
        let summarize_hook = summarizer.summarize_hook();

        for _ in 0..NUM_COMPLETIONS_FOR_SUMMARY {
            summarize_hook(mock_context.clone()).await.unwrap();
        }
        // Validate that the completion summary logic executes after enough completions
        assert_eq!(mock_context.history().await, vec!["Mock Summary"]);
    }

    #[derive(Clone)]
    struct MockContext {
        messages: Arc<std::sync::Mutex<Vec<String>>>,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                messages: Arc::new(std::sync::Mutex::new(Vec::new())),
            }
        }

        async fn add_message(&self, message: ChatMessage) {
            self.messages.lock().unwrap().push(message.to_text().to_string());
        }

        async fn history(&self) -> Vec<String> {
            self.messages.lock().unwrap().clone()
        }
    }

    impl AfterEachFn for MockContext {
        fn call_box(&self) -> BoxFuture<'_, Result<(), Box<dyn std::error::Error + Send>>> {
            Box::pin(async { Ok(()) })
        }
    }
}
