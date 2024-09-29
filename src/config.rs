use anyhow::{Context as _, Result};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use swiftide::integrations;
use swiftide::traits::SimplePrompt;
use swiftide::{integrations::treesitter::SupportedLanguages, traits::EmbeddingModel};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub language: SupportedLanguages,
    pub llm: LLMConfigurations,
}
impl Config {
    /// Loads the configuration file from the current path
    pub(crate) async fn load() -> Result<Config> {
        let file = tokio::fs::read("kwaak.toml").await?;

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
}

impl TryInto<Box<dyn EmbeddingModel>> for &LLMConfiguration {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<Box<dyn EmbeddingModel>, Self::Error> {
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
                            .ok_or(anyhow::anyhow!("Missing prompt model"))?,
                    )
                    .build()?,
            ),
            _ => unimplemented!(),
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
                            .ok_or(anyhow::anyhow!("Missing prompt model"))?,
                    )
                    .build()?,
            ),
            _ => unimplemented!(),
        };
        Ok(boxed)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LLMConfigurations {
    Single(LLMConfiguration),
    Multiple {
        indexing: LLMConfiguration,
        embedding: LLMConfiguration,
        query: LLMConfiguration,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "provider")]
pub enum LLMConfiguration {
    OpenAI {
        api_key: SecretString,
        prompt_model: Option<String>,
        embedding_model: Option<String>,
    },
    Groq {
        api_key: SecretString,
        prompt_model: String,
    },
    Ollama {
        prompt_model: Option<String>,
        embedding_model: Option<String>,
    },
    AWSBedrock {
        prompt_model: String,
    },
    FastEmbed {
        embedding_model: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::ExposeSecret;
    use swiftide::integrations::treesitter::SupportedLanguages;

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
            assert_eq!(prompt_model.as_deref(), Some("gpt-4o-mini"));
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
            provider = "FastEmbed"
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
                assert_eq!(prompt_model.as_deref(), Some("gpt-4o-mini"));
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
                assert_eq!(prompt_model.as_deref(), Some("gpt-4o-mini"));
            } else {
                panic!("Expected OpenAI configuration for query");
            }

            if let LLMConfiguration::FastEmbed { embedding_model } = embedding {
                assert!(embedding_model.is_none());
            } else {
                panic!("Expected FastEmbed configuration for embedding");
            }
        } else {
            panic!("Expected multiple LLM configurations");
        }
    }
}
