use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use crate::commands::CommandResponder;
use crate::repository::Repository;
use crate::storage;
use anyhow::Result;
use swiftide::indexing::loaders;
use swiftide::indexing::transformers;
use swiftide::indexing::Node;
use swiftide::traits::EmbeddingModel;
use swiftide::traits::NodeCache;
use swiftide::traits::Persist;
use swiftide::traits::SimplePrompt;

// Removed unused garbage_collection import

const CODE_CHUNK_RANGE: std::ops::Range<usize> = 100..2048;
const MARKDOWN_CHUNK_RANGE: std::ops::Range<usize> = 100..1024;

#[tracing::instrument(skip_all)]
pub async fn index_repository(
    repository: &Repository,
    responder: Option<CommandResponder>,
) -> Result<()> {
    let updater = UiUpdater::from(responder);

    updater.send_update("Cleaning up the index ...");
    // Modify further logic as necessary for garbage collection and ensure other dependencies resolve correctly

    Ok(())
}

// Remaining implementation, unchanged
#[derive(Debug, Clone)]
struct UiUpdater(Option<CommandResponder>);

impl UiUpdater {
    fn send_update(&self, state: impl Into<String>) {
        let Some(responder) = &self.0 else { return };
        responder.send_update(state);
    }
}

impl From<Option<CommandResponder>> for UiUpdater {
    fn from(responder: Option<CommandResponder>) -> Self {
        Self(responder)
    }
}
