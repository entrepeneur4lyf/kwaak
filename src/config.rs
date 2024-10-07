use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use swiftide::integrations;
use swiftide::traits::SimplePrompt;
use swiftide::{integrations::treesitter::SupportedLanguages, traits::EmbeddingModel};

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

fn default_project_name() -> String {
    // Infer from the current directory
    std::env::current_dir()
        .expect("Failed to get current directory")
        .file_name()
        .expect("Failed to get current directory name")
        .to_string_lossy()
        .to_string()
}

fn default_cache_dir() -> PathBuf {
    let mut path = dirs::cache_dir().expect("Failed to get cache directory");
    path.push("kwaak");
    path
}

fn default_log_dir() -> PathBuf {
    let mut path = dirs::cache_dir().expect("Failed to get cache directory");
    path.push("kwaak");
    path.push("logs");

    path
}

fn default_openai_api_key() -> SecretString {
    std::env::var("OPENAI_API_KEY")
        .map(SecretString::from)
        .expect("Missing OPENAI_API_KEY environment variable or config")
}

impl TryInto<Box<dyn EmbeddingModel>> for &LLMConfiguration {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<Box<dyn EmbeddingModel>, Self::Error> {
        let boxed = match self {
            LLMConfiguration::OpenAI {
                api_key,
                embedding_model,
                ..
            } => Box::new(
                integrations::openai::OpenAI::builder()
                    .client(async_openai::Client::with_config(
                        async_openai::config::OpenAIConfig::default()
                            .with_api_key(api_key.expose_secret()),
                    ))
                    .default_embed_model(
                        embedding_model
                            .as_ref()
                            .ok_or(anyhow::anyhow!("Missing prompt model"))?
                            .to_string(),
                    )
                    .build()?,
            ),
        };

        Ok(boxed)
    }
}

impl TryInto<Box<dyn SimplePrompt>> for &LLMConfiguration {
    type Error = anyhow::Error;
    fn try_into(self) -> std::result::Result<Box<dyn SimplePrompt>, Self::Error> {
        let boxed = match self {
            LLMConfiguration::OpenAI {
                api_key,
                prompt_model,
                ..
            } => Box::new(
                integrations::openai::OpenAI::builder()
                    .client(async_openai::Client::with_config(
                        async_openai::config::OpenAIConfig::default()
                            .with_api_key(api_key.expose_secret()),
                    ))
                    .default_prompt_model(
                        prompt_model
                            .as_ref()
                            .ok_or(anyhow::anyhow!("Missing prompt model"))?
                            .to_string(),
                    )
                    .build()?,
            ),
        };
        Ok(boxed)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LLMConfigurations {
    Single(LLMConfiguration),
    Multiple {
        indexing: LLMConfiguration,
        embedding: LLMConfiguration,
        query: LLMConfiguration,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "provider")]
pub enum LLMConfiguration {
    OpenAI {
        #[serde(
            default = "default_openai_api_key",
            serialize_with = "serde_hidden_secret"
        )]
        api_key: SecretString,
        prompt_model: Option<OpenAIPromptModel>,
        embedding_model: Option<OpenAIEmbeddingModel>,
    },
    // Groq {
    //     api_key: SecretString,
    //     prompt_model: String,
    // },
    // Ollama {
    //     prompt_model: Option<String>,
    //     embedding_model: Option<String>,
    //     vector_size: Option<usize>,
    // },
    // AWSBedrock {
    //     prompt_model: String,
    // },
    // FastEmbed {
    //     embedding_model: String,
    //     vector_size: usize,
    // },
}

fn serde_hidden_secret<S>(_secret: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str("****")
}

impl LLMConfiguration {
    pub(crate) fn vector_size(&self) -> Result<i32> {
        match self {
            LLMConfiguration::OpenAI {
                embedding_model, ..
            } => {
                let model = embedding_model
                    .as_ref()
                    .ok_or(anyhow::anyhow!("Missing embedding model"))?;
                match model {
                    OpenAIEmbeddingModel::TextEmbedding3Small => Ok(1536),
                    OpenAIEmbeddingModel::TextEmbedding3Large => Ok(3072),
                }
            }
        }
    }
}

#[derive(
    Debug, Clone, Deserialize, Serialize, PartialEq, strum_macros::EnumString, strum_macros::Display,
)]
pub enum OpenAIPromptModel {
    #[strum(serialize = "gpt-4o-mini")]
    #[serde(rename = "gpt-4o-mini")]
    GPT4OMini,
    #[strum(serialize = "gpt-4o")]
    #[serde(rename = "gpt-4o")]
    GPT4O,
}

#[derive(
    Debug, Clone, Deserialize, Serialize, strum_macros::EnumString, strum_macros::Display, PartialEq,
)]
pub enum OpenAIEmbeddingModel {
    #[strum(serialize = "text-embedding-3-small")]
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[strum(serialize = "text-embedding-3-large")]
    #[serde(rename = "text-embedding")]
    TextEmbedding3Large,
}

#[cfg(test)]
mod tests {
    #![allow(irrefutable_let_patterns)]
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
