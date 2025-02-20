mod conversation_summarizer;
mod delegating_agent;
pub mod env_setup;
pub mod running_agent;
pub mod session;
mod tool_summarizer;
pub mod tools;
mod util;
pub mod v1;
use session::{RunningSession, Session};
use std::sync::Arc;

use anyhow::Result;

/// NOTE: On architecture, when more agents are added, it would be nice to have the concept of an
/// (Agent/Chat) session that wraps all this complexity => Responders then update on the session.
/// Makes everything a lot simpler. The session can then also references the running agent,
/// executor, etc

#[tracing::instrument(skip(repository, command_responder))]
pub async fn start_session(
    uuid: Uuid,
    repository: &Repository,
    initial_query: &str,
    command_responder: Arc<dyn Responder>,
) -> Result<RunningSession> {
    command_responder.update("starting up agent for the first time, this might take a while");

    Session::builder()
        .session_id(uuid)
        .repository(repository.clone())
        .default_responder(command_responder)
        .initial_query(initial_query.to_string())
        .start()
        .await
}

use uuid::Uuid;

use crate::{commands::Responder, git::github::GithubSession, repository::Repository};
