mod agent;
mod chat;
mod chat_message;
mod cli;
mod commands;
// Export frontend module
pub mod frontend;
mod git;
mod indexing;
mod kwaak_tracing;
mod onboarding;
mod repository;
mod runtime_settings;
mod storage;
mod templates;
mod test_utils;
mod util;

// Re-export frontend components
pub use frontend::*;
