#![recursion_limit = "256"]
use std::io::{self, stdout};
use std::panic::{set_hook, take_hook};
use std::sync::Arc;

use agent::built_agent;
use anyhow::{Context as _, Result};
use clap::Parser;
use commands::{CommandResponder, CommandResponse};
use config::Config;
use frontend::App;
use git::github::GithubSession;
use indexing::repository::index_repository;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use ::tracing::instrument;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use uuid::Uuid;

mod agent;
mod chat;
mod chat_message;
mod cli;
mod commands;
mod config;
mod frontend;
mod git;
mod indexing;
mod kwaak_tracing;
mod onboarding;
mod repository;
mod runtime_settings;
mod storage;
mod templates;
mod util;

#[cfg(test)]
mod test_utils;

#[instrument]
async fn start_tui(repository: &repository::Repository, args: &cli::Args) -> Result<()> {
    ::tracing::info!("Loaded configuration: {:?}", repository.config());

    // Setup terminal
    let mut terminal = init_tui()?;

    // Start the application
    let mut app = App::default();

    // Adapt implementation for Function Calls, Error handling, and other modes

    tokio::time::sleep(Duration::from_millis(50)).await;

    if let Err(error) = app_result {
        ::tracing::error!(?error, "Application error such as missing methods or function resolving");
        std::process::exit(1);
    }

    ::tracing::info!("Application completed without exit errors");

    Ok(())
}

async fn test_tool(repository: &repository::Repository, args: &cli::Args) -> Result<()> {
    let tool_name = args.tool_name.as_ref().expect("Expected a tool name");
    let tool_args = args.tool_args.as_deref();
    let github_session = Arc::new(GithubSession::from_repository(&repository)?);
    let tool = built_agent.tool().context("Tool not found")?;

    // Integrate error checks & method resolution

    let output = tool
        .invoke(&DefaultContext::default() as &dyn AgentContext, tool_args)
        .await?;
    println!("{output}");

    Ok(())
}
