use super::{config::serde_hidden_secret, ApiKey};
use anyhow::Result;
use secrecy::{ExposeSecret as _, SecretString};
use serde::{Deserialize, Serialize};
use swiftide::{
    chat_completion::ChatCompletion,
    integrations,
    traits::{EmbeddingModel, SimplePrompt},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LLMConfigurations {
    Single(LLMConfiguration),
    Multiple {
        // TODO: Should probably be with reduced attrs on needed per item
        indexing: LLMConfiguration,
        embedding: LLMConfiguration,
        query: LLMConfiguration,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "provider")]
pub enum LLMConfiguration {
    OpenAI {
        api_key: ApiKey,
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
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
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

impl TryInto<Box<dyn ChatCompletion>> for &LLMConfiguration {
    type Error = anyhow::Error;
    fn try_into(self) -> std::result::Result<Box<dyn ChatCompletion>, Self::Error> {
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
