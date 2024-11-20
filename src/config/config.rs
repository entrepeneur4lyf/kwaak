use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use swiftide::chat_completion::ChatCompletion;
use swiftide::integrations;
use swiftide::traits::SimplePrompt;
use swiftide::{integrations::treesitter::SupportedLanguages, traits::EmbeddingModel};

use super::defaults::*;
use super::{LLMConfiguration, LLMConfigurations};

// TODO: Improving parsing by enforcing invariants
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_project_name")]
    pub project_name: String,
    pub language: SupportedLanguages,
    pub llm: LLMConfigurations,
    #[serde(default = "default_cache_dir")]
    cache_dir: PathBuf,
    #[serde(default = "default_log_dir")]
    log_dir: PathBuf,

    pub docker: DockerConfiguration,

    #[serde(
        serialize_with = "serde_hidden_secret",
        default = "default_github_token"
    )]
    pub github_token: SecretString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerConfiguration {
    #[serde(default = "default_dockerfile")]
    pub dockerfile: PathBuf,
    #[serde(default = "default_docker_context")]
    pub context: PathBuf,
}

impl Config {
    /// Loads the configuration file from the current path
    pub(crate) async fn load() -> Result<Config> {
        let file = tokio::fs::read("kwaak.toml")
            .await
            .context("Could not find `kwaak.toml` in current directory")?;

        toml::from_str(std::str::from_utf8(&file)?).context("Failed to parse configuration")
    }

    pub fn indexing_provider(&self) -> &LLMConfiguration {
        match &self.llm {
            LLMConfigurations::Single(config) => config,
            LLMConfigurations::Multiple { indexing, .. } => indexing,
        }
    }

    pub fn embedding_provider(&self) -> &LLMConfiguration {
        match &self.llm {
            LLMConfigurations::Single(config) => config,
            LLMConfigurations::Multiple { embedding, .. } => embedding,
        }
    }

    pub fn query_provider(&self) -> &LLMConfiguration {
        match &self.llm {
            LLMConfigurations::Single(config) => config,
            LLMConfigurations::Multiple { query, .. } => query,
        }
    }

    pub fn cache_dir(&self) -> &Path {
        self.cache_dir.as_path()
    }

    pub fn log_dir(&self) -> &Path {
        self.log_dir.as_path()
    }
}

/// Serialize a secret as "****"
pub(super) fn serde_hidden_secret<S>(
    _secret: &SecretString,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str("****")
}

#[cfg(test)]
mod tests {
    #![allow(irrefutable_let_patterns)]
    use crate::config::{OpenAIEmbeddingModel, OpenAIPromptModel};

    use super::*;
    use secrecy::ExposeSecret;
    use swiftide::integrations::treesitter::SupportedLanguages;

    #[test]
    fn test_default_project_name() {
        let project_name = default_project_name();
        assert_eq!(project_name, "kwaak");
    }

    #[test]
    fn test_deserialize_toml_single() {
        let toml = r#"
            language = "rust"

            [llm]
            provider = "OpenAI"
            api_key = "test-key"
            prompt_model = "gpt-4o-mini"
            "#;

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.language, SupportedLanguages::Rust);

        if let LLMConfigurations::Single(LLMConfiguration::OpenAI {
            api_key,
            prompt_model,
            ..
        }) = &config.llm
        {
            assert_eq!(api_key.expose_secret(), "test-key");
            assert_eq!(prompt_model, &Some(OpenAIPromptModel::GPT4OMini));
        } else {
            panic!("Expected single OpenAI configuration");
        }
    }

    #[test]
    fn test_deserialize_toml_multiple() {
        let toml = r#"
            language = "rust"

            [llm.indexing]
            provider = "OpenAI"
            api_key = "test-key"
            prompt_model = "gpt-4o-mini"

            [llm.query]
            provider = "OpenAI"
            api_key = "other-test-key"
            prompt_model = "gpt-4o-mini"

            [llm.embedding]
            provider = "OpenAI"
            api_key = "other-test-key"
            embedding_model = "text-embedding-3-small"
            "#;

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.language, SupportedLanguages::Rust);

        if let LLMConfigurations::Multiple {
            indexing,
            embedding,
            query,
        } = &config.llm
        {
            if let LLMConfiguration::OpenAI {
                api_key,
                prompt_model,
                ..
            } = indexing
            {
                assert_eq!(api_key.expose_secret(), "test-key");
                assert_eq!(prompt_model, &Some(OpenAIPromptModel::GPT4OMini));
            } else {
                panic!("Expected OpenAI configuration for indexing");
            }

            if let LLMConfiguration::OpenAI {
                api_key,
                prompt_model,
                ..
            } = query
            {
                assert_eq!(api_key.expose_secret(), "other-test-key");
                assert_eq!(prompt_model, &Some(OpenAIPromptModel::GPT4OMini));
            } else {
                panic!("Expected OpenAI configuration for query");
            }

            if let LLMConfiguration::OpenAI {
                api_key,
                embedding_model,
                ..
            } = embedding
            {
                assert_eq!(api_key.expose_secret(), "other-test-key");
                assert_eq!(
                    embedding_model,
                    &Some(OpenAIEmbeddingModel::TextEmbedding3Small)
                );
            }
        } else {
            panic!("Expected multiple LLM configurations");
        }
    }
}
