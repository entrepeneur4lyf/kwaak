use secrecy::SecretString;
use serde::Deserialize;
use swiftide::integrations::treesitter::SupportedLanguages;

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub language: SupportedLanguages,
    pub llm: LLMConfigurations,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum LLMConfigurations {
    Single(LLMConfiguration),
    Multiple {
        indexing: LLMConfiguration,
        embedding: LLMConfiguration,
        query: LLMConfiguration,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "provider")]
enum LLMConfiguration {
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
