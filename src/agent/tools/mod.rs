mod delegate_agent;
mod github;
mod replace_lines;

pub use delegate_agent::DelegateAgent;
pub use github::*;
pub use replace_lines::replace_lines;

use std::sync::Arc;
use swiftide::traits::CommandError;

use anyhow::{Context as _, Result};
use swiftide::{
    chat_completion::{errors::ToolError, ToolOutput},
    query::{search_strategies, states},
    traits::{AgentContext, Command},
};
use swiftide_macros::{tool, Tool};
use tavily::Tavily;
use tokio::sync::Mutex;

use crate::{
    config::ApiKey,
    templates::Templates,
    util::{self, accept_non_zero_exit},
};

#[allow(dead_code)]
static MAIN_BRANCH_CMD: &str = "git remote show origin | sed -n '/HEAD branch/s/.*: //p'";

/// WARN: Experimental
#[tool(
    description = "Run any shell command in the current project, use this if other tools are not enough.",
    param(
        name = "cmd",
        description = "The shell command, including any arguments if needed, to run"
    )
)]
pub async fn shell_command(context: &dyn AgentContext, cmd: &str) -> Result<ToolOutput, ToolError> {
    if util::is_git_branch_change(cmd) {
        return Ok(
            "You cannot change branches, you are already on a branch created specifically for you."
                .into(),
        );
    }
    let output = accept_non_zero_exit(context.exec_cmd(&Command::Shell(cmd.into())).await)?;
    Ok(output.into())
}

#[tool(
    description = "Reads file content",
    param(name = "file_name", description = "Full path of the file")
)]
pub async fn read_file(
    context: &dyn AgentContext,
    file_name: &str,
) -> Result<ToolOutput, ToolError> {
    let cmd = Command::ReadFile(file_name.into());

    // i.e. if the file doesn't exist, just forward that message
    let output = accept_non_zero_exit(context.exec_cmd(&cmd).await)?;

    Ok(output.into())
}

// TODO: Better to have a single read_file tool with an optional line number flag
#[tool(
    description = "Reads file content, including line numbers. You MUST use this tool to retrieve line numbers before making an edit with edit_file",
    param(name = "file_name", description = "Full path of the file")
)]
pub async fn read_file_with_line_numbers(
    context: &dyn AgentContext,
    file_name: &str,
) -> Result<ToolOutput, ToolError> {
    let cmd = Command::ReadFile(file_name.into());

    // i.e. if the file doesn't exist, just forward that message
    let output = accept_non_zero_exit(context.exec_cmd(&cmd).await)?;

    let lines = output
        .output
        .lines()
        .enumerate()
        .map(|(i, l)| format!("{line_num}|{l}", line_num = i + 1));

    Ok(lines.collect::<Vec<_>>().join("\n").into())
}

#[tool(
    description = "Write to a file. You MUST ALWAYS include the full file content, including what you did not change, as it overwrites the full file. Only make changes that pertain to your task.",
    param(name = "file_name", description = "Full path of the file"),
    param(name = "content", description = "FULL Content to write to the file")
)]
pub async fn write_file(
    context: &dyn AgentContext,
    file_name: &str,
    content: &str,
) -> Result<ToolOutput, ToolError> {
    let cmd = Command::WriteFile(file_name.into(), content.into());

    context.exec_cmd(&cmd).await?;

    let success_message = format!("File written successfully to {file_name}");

    Ok(success_message.into())
}

#[tool(
    description = "Searches for a file inside the current project, leave the argument empty to list all files. Uses `find`.",
    param(name = "file_name", description = "Partial or full name of the file")
)]
pub async fn search_file(
    context: &dyn AgentContext,
    file_name: &str,
) -> Result<ToolOutput, ToolError> {
    let cmd = Command::Shell(format!("fd -E '.git/*' -iH --full-path '{file_name}'"));
    let output = accept_non_zero_exit(context.exec_cmd(&cmd).await)?;

    Ok(output.into())
}

#[tool(
    description = "Invoke a git command on the current repository",
    param(name = "command", description = "Git sub-command to run")
)]
pub async fn git(context: &dyn AgentContext, command: &str) -> Result<ToolOutput, ToolError> {
    let cmd = format!("git {command}");
    if util::is_git_branch_change(&cmd) {
        return Ok(
            "You cannot change branches, you are already on a branch created specifically for you."
                .into(),
        );
    }
    let cmd = Command::Shell(cmd);
    let output = accept_non_zero_exit(context.exec_cmd(&cmd).await)?;

    Ok(output.into())
}

#[derive(Tool, Clone)]
#[tool(
    description = "Reset changes you have made to a file. If you have made changes to a file and need to reset them, use this tool.",
    param(name = "file_name", description = "Full path of the file")
)]
pub struct ResetFile {
    start_ref: String,
}

impl ResetFile {
    pub fn new(start_ref: impl AsRef<str>) -> Self {
        Self {
            start_ref: start_ref.as_ref().to_string(),
        }
    }
    pub async fn reset_file(
        &self,
        context: &dyn AgentContext,
        file_name: &str,
    ) -> Result<ToolOutput, ToolError> {
        let cmd = Command::Shell(format!(
            "git checkout {start_ref} -- {file_name}",
            start_ref = self.start_ref
        ));

        let output = accept_non_zero_exit(context.exec_cmd(&cmd).await)?;

        Ok(output.into())
    }
}

#[tool(
    description = "Search code in the project with ripgrep. Only searches within the current project. For searching code outside the project, use other tools instead.",
    param(
        name = "query",
        description = "Code you would like to find in the repository. Best used for exact search in the code. Uses `ripgrep`."
    )
)]
pub async fn search_code(context: &dyn AgentContext, query: &str) -> Result<ToolOutput, ToolError> {
    let cmd = Command::Shell(format!("rg -g '!.git' -i. -F '{query}'"));
    let output = accept_non_zero_exit(context.exec_cmd(&cmd).await)?;
    Ok(output.into())
}

#[derive(Tool, Clone)]
#[tool(
    description = "Search code and documentation in human language in the project. Only searches within the current project. If you need help on code outside the project, use other tools.",
    param(
        name = "query",
        description = "A description, question, or literal code you want to know more about. Uses a semantic similarly search."
    )
)]
pub struct ExplainCode<'a> {
    query_pipeline: Arc<
        Mutex<
            swiftide::query::Pipeline<
                'a,
                search_strategies::SimilaritySingleEmbedding,
                states::Answered,
            >,
        >,
    >,
}

impl<'a> ExplainCode<'a> {
    #[must_use]
    pub fn new(
        query_pipeline: swiftide::query::Pipeline<
            'a,
            search_strategies::SimilaritySingleEmbedding,
            states::Answered,
        >,
    ) -> Self {
        Self {
            query_pipeline: Arc::new(Mutex::new(query_pipeline)),
        }
    }
    async fn explain_code(
        &self,
        _context: &dyn AgentContext,
        query: &str,
    ) -> Result<ToolOutput, ToolError> {
        let results = self
            .query_pipeline
            .lock()
            .await
            .query_mut(query)
            .await?
            .answer()
            .to_string();
        Ok(results.into())
    }
}

#[derive(Tool, Clone, Debug)]
#[tool(
    description = "Runs tests in the current project. Run this in favour of coverage, as it is typically faster."
)]
pub struct RunTests {
    pub test_command: String,
}

impl RunTests {
    pub fn new(test_command: impl AsRef<str>) -> Self {
        Self {
            test_command: test_command.as_ref().to_string(),
        }
    }

    async fn run_tests(&self, context: &dyn AgentContext) -> Result<ToolOutput, ToolError> {
        let cmd = Command::Shell(self.test_command.clone());
        let output = accept_non_zero_exit(context.exec_cmd(&cmd).await)?;

        Ok(output.into())
    }
}

#[derive(Tool, Clone, Debug)]
#[tool(
    description = "Get coverage of tests, this also runs the tests. Only run this in favour of just the tests if you need coverage, as it is typically slower than running tests."
)]
pub struct RunCoverage {
    pub coverage_command: String,
}

impl RunCoverage {
    pub fn new(coverage_command: impl AsRef<str>) -> Self {
        Self {
            coverage_command: coverage_command.as_ref().to_string(),
        }
    }

    async fn run_coverage(&self, context: &dyn AgentContext) -> Result<ToolOutput, ToolError> {
        let cmd = Command::Shell(self.coverage_command.clone());
        let output = accept_non_zero_exit(context.exec_cmd(&cmd).await)?;

        Ok(output.into())
    }
}

#[derive(Tool, Clone)]
#[tool(
    description = "Search the web to answer a question. If you encounter an issue that cannot be resolved, use this tool to help getting an answer.",
    param(name = "query", description = "Search query")
)]
pub struct SearchWeb {
    tavily_client: Arc<Tavily>,
    api_key: ApiKey,
}

impl SearchWeb {
    #[must_use]
    pub fn new(tavily_client: Tavily, api_key: ApiKey) -> Self {
        Self {
            tavily_client: Arc::new(tavily_client),
            api_key,
        }
    }
    async fn search_web(
        &self,
        _context: &dyn AgentContext,
        query: &str,
    ) -> Result<ToolOutput, ToolError> {
        let request = tavily::SearchRequest::new(self.api_key.expose_secret(), query)
            .search_depth("advanced")
            .include_answer(true)
            .include_images(false)
            .include_raw_content(false)
            .max_results(5);

        let results = self
            .tavily_client
            .call(&request)
            .await
            .map_err(anyhow::Error::from)?;

        tracing::debug!(results = ?results, "Search results from tavily");

        let mut context = tera::Context::new();

        context.insert("answer", &results.answer);
        context.insert(
            "results",
            &results
                .results
                .iter()
                .filter(|r| r.score >= 0.5)
                .map(|r| {
                    serde_json::json!({
                        "title": r.title,
                        "content": r.content,
                        "url": r.url,
                    })
                })
                .collect::<Vec<_>>(),
        );
        context.insert("follow_up_questions", &results.follow_up_questions);

        let rendered = Templates::render("tavily_search_results.md", &context)
            .context("Failed to render search web results")?;

        Ok(rendered.into())
    }
}

#[tool(
    description = "Fetch a url and present it as markdown. Useful for fetching content from the web like documentation, code, snippetes, etc. Will also include links and can be used to deeply explore a subject that otherwise cannot be explored.",
    param(name = "url", description = "The url to fetch")
)]
pub async fn fetch_url(_context: &dyn AgentContext, url: &str) -> Result<ToolOutput, ToolError> {
    let url_content = match reqwest::get(url).await {
        Ok(response) if response.status().is_success() => response.text().await.unwrap(),

        // Assuming 9/10 parsing/network errors for now always return it to the llm
        Err(e) => return Ok(format!("Failed to fetch url: {e:#}").into()),
        Ok(response) => return Ok(format!("Failed to fetch url: {}", response.status()).into()),
    };

    htmd::HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style", "img", "video", "audio", "embed"])
        .build()
        .convert(&url_content)
        .or_else(|e| {
            tracing::warn!("Error converting markdown {e:#}");
            Ok(url_content)
        })
        .map(Into::into)
}

#[tool(
    description = "Add new lines after a specific line number. You MUST read the file with line numbers first BEFORE EVERY EDIT, to know after what line number to add. After adding lines, you MUST read the file again to get the new line numbers.",
    param(name = "file_name", description = "Full path of the file"),
    param(
        name = "start_line",
        description = "The line number to insert the content after"
    ),
    param(name = "content", description = "New content")
)]
pub async fn add_lines(
    context: &dyn AgentContext,
    file_name: &str,
    start_line: &str,
    content: &str,
) -> Result<ToolOutput, ToolError> {
    // Read the file content
    let cmd = Command::ReadFile(file_name.into());

    let file_content = match context.exec_cmd(&cmd).await {
        Ok(output) => output.output,
        Err(CommandError::NonZeroExit(output, ..)) => {
            return Ok(output.into());
        }
        Err(e) => return Err(e.into()),
    };

    let mut lines = file_content.lines().collect::<Vec<_>>();

    let Ok(start_line) = start_line.parse::<usize>() else {
        return Ok("Invalid start line number, must be a valid number greater than 0".into());
    };

    let lines_len = lines.len();

    if start_line > lines_len {
        return Ok("Start or end line number is out of bounds".into());
    }

    if start_line == 0 {
        return Ok("Start line number must be greater than 0".into());
    }

    // Input is 1 indexed, lines are 0 indexed
    lines.insert(start_line, content);

    let write_cmd = Command::WriteFile(file_name.into(), lines.join("\n"));
    context.exec_cmd(&write_cmd).await?;

    Ok(format!("Successfully added content to {file_name} at line {start_line}. Before making new edits, you MUST read the file again, as the line numbers WILL have changed.").into())
}
