use std::{collections::HashMap, sync::Arc, time::Duration};
use anyhow::Resu..._config::DockerConfiguration;
use crate::config::{Config, LLMConfigurations, OpenAIPromptModel};
use crate::{
    agent, 
    chat_message::ChatMessage, 
    frontend::{App, UIEvent}, 
    indexing, 
    repository::Repository,
};
use swiftide::integrations::treesitter::SupportedLanguages;
use tokio::{sync::{mpsc, Mutex, RwLock}, task};
use uuid::Uuid;...
