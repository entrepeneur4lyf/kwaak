//! Builds various storage providers for kwaak

use anyhow::Result;
use swiftide::integrations::lancedb::LanceDB;

use crate::repository::Repository;

pub fn build_lancedb(repository: &Repository) -> Result<LanceDB> {
    let config = repository.config();
    let mut cache_dir = config.cache_dir();
    cache_dir.push("lancedb");

    let embedding_provider = config.embedding_provider();

    LanceDB::builder()
        .uri(
            cache_dir
                .to_str()
                .ok_or(anyhow::anyhow!("Failed to convert path to string"))?,
        )
        .vector_size(embedding_provider.vector_size()?)
        .table_name(&config.project_name)
        .build()
}
