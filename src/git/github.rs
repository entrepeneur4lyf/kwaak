//! This module provides a github session wrapping octocrab
//!
//! It is responsible for providing tooling and interaction with github
use std::sync::Mutex;

use anyhow::{Context, Result};
use octocrab::{models::pulls::PullRequest, Octocrab};
use secrecy::SecretString;
use swiftide::chat_completion::ChatMessage;
use reqwest::Client;
use base64::{engine::general_purpose, Engine as _};

use crate::{config::ApiKey, repository::Repository};

#[derive(Debug)]
pub struct GithubSession {
    token: ApiKey,
    octocrab: Octocrab,
    repository: Repository,
    active_pull_request: Mutex<Option<PullRequest>>,
}
impl GithubSession {
    pub fn from_repository(repository: &Repository) -> Result<Self> {
        let token = repository
            .config()
            .github
            .token
            .clone()
            .ok_or(anyhow::anyhow!("No github token found in config"))?;

        let octocrab = Octocrab::builder()
            .personal_token(token.expose_secret())
            .build()?;

        Ok(Self {
            token,
            octocrab,
            repository: repository.to_owned(),
            active_pull_request: Mutex::new(None),
        })
    }

    /// Adds the github token to the repository url
    ///
    /// Used to overwrite the origin remote so that the agent can interact with git
    #[tracing::instrument(skip_all)]
    pub fn add_token_to_url(&self, repo_url: impl AsRef<str>) -> Result<SecretString> {
        if !repo_url.as_ref().starts_with("https://") {
            anyhow::bail!("Only https urls are supported")
        }

        let mut parsed = url::Url::parse(repo_url.as_ref()).context("Failed to parse url")?;

        parsed
            .set_username("x-access-token")
            .and_then(|()| parsed.set_password(Some(self.token.expose_secret())))
            .expect("Infallible");

        Ok(SecretString::from(parsed.to_string()))
    }

    pub fn main_branch(&self) -> &str {
        &self.repository.config().github.main_branch
    }

    #[tracing::instrument(skip_all)]
    pub async fn create_or_update_pull_request(
        &self,
        branch_name: impl AsRef<str>,
        base_branch_name: impl AsRef<str>,
        title: impl AsRef<str>,
        description: impl AsRef<str>,
        messages: &[ChatMessage],
    ) -> Result<PullRequest> {
        let owner = &self.repository.config().github.owner;
        let repo = &self.repository.config().github.repository;

        // Upload messages to a file on GitHub
        let file_path = format!("messages/{}_messages.md", branch_name.as_ref());
        let serialized_messages = serde_json::to_string(&messages.iter().map(format_message).collect::<Vec<_>>())?;
        self.upload_file_to_github(owner, repo, &file_path, &serialized_messages).await?;

        let body = format!(
            "{}\n\nMessages have been saved to [{}](https://github.com/{}/{}/blob/{}/{})",
            description.as_ref(),
            file_path,
            owner,
            repo,
            branch_name.as_ref(),
            file_path
        );

        let maybe_pull = { self.active_pull_request.lock().unwrap().clone() };

        if let Some(pull_request) = maybe_pull {
            let pull_request = self
                .octocrab
                .pulls(owner, repo)
                .update(pull_request.number)
                .title(title.as_ref())
                .body(&body)
                .send()
                .await?;

            self.active_pull_request
                .lock()
                .unwrap()
                .replace(pull_request.clone());

            return Ok(pull_request);
        }

        let pull_request = self
            .octocrab
            .pulls(owner, repo)
            .create(
                title.as_ref(),
                branch_name.as_ref(),
                base_branch_name.as_ref(),
            )
            .body(&body)
            .send()
            .await?;

        self.active_pull_request
            .lock()
            .unwrap()
            .replace(pull_request.clone());

        Ok(pull_request)
    }

    /// Upload a file to the GitHub repository
    async fn upload_file_to_github(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        content: &str,
    ) -> Result<()> {
        let client = Client::new();
        let url = format!("https://api.github.com/repos/{owner}/{repo}/contents/{path}");
        let payload = serde_json::json!({
            "message": "Upload messages",
            "content": general_purpose::STANDARD.encode(content),
        });

        let response = client
            .put(&url)
            .header("Authorization", format!("token {}", self.token.expose_secret()))
            .header("Accept", "application/vnd.github.v3+json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to upload file to GitHub: {}", response.text().await?);
        }

        Ok(())
    }
}

fn format_message(message: &ChatMessage) -> serde_json::Value {
    let role = match message {
        ChatMessage::User(_) => "\u{25b6} User",
        ChatMessage::System(_) => "\u{2139} System",
        // Add a nice uncoloured glyph for the summary
        ChatMessage::Summary(_) => ">> Summary",
        ChatMessage::Assistant(..) => "\u{2726} Assistant",
        ChatMessage::ToolOutput(..) => "\u{2699} Tool Output",
    };
    let content = match message {
        ChatMessage::User(msg) | ChatMessage::System(msg) | ChatMessage::Summary(msg) => {
            msg.to_string()
        }
        ChatMessage::Assistant(msg, tool_calls) => {
            let mut msg = msg.as_deref().unwrap_or_default().to_string();

            if let Some(tool_calls) = tool_calls {
                msg.push_str("\nTool calls: \n");
                for tool_call in tool_calls {
                    msg.push_str(&format!("{tool_call}\n"));
                }
            }

            msg
        }
        ChatMessage::ToolOutput(tool_call, tool_output) => {
            format!("{tool_call} => {tool_output}")
        }
    };

    serde_json::json!({
        "role": role,
        "content": content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_render() {
        let chat_messages = vec![
            ChatMessage::new_user("user message"),
            ChatMessage::new_system("system message"),
            ChatMessage::new_assistant(Some("assistant message"), None),
            ChatMessage::new_summary("summary message"),
        ];

        let context = tera::Context::from_serialize(serde_json::json!({
            "owner": "owner",
            "repo": "repo",
            "branch_name": "branch_name",
            "base_branch_name": "base_branch_name",
            "title": "title",
            "description": "description",
            "messages": chat_messages.iter().map(format_message).collect::<Vec<_>>(),


        }))
        .unwrap();
        let rendered = Templates::render("pull_request.md", &context).unwrap();

        insta::assert_snapshot!(rendered);
    }
}
