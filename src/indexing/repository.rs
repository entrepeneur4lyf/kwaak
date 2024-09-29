use crate::repository::Repository;
use anyhow::Result;
use swiftide::indexing::loaders;
use swiftide::indexing::transformers;
use swiftide::integrations;
use swiftide::traits::EmbeddingModel;
use swiftide::traits::SimplePrompt;

pub async fn index_repository(repository: &Repository) -> Result<()> {
    let extensions = repository.config().language.file_extensions();
    let loader = loaders::FileLoader::new(repository.path()).with_extensions(extensions);
    // NOTE: Parameter to optimize on
    let chunk_size = 100..2048;

    // TODO: Needs configuration and not set here
    let lancedb = integrations::lancedb::LanceDB::builder()
        .uri("/my/lancedb")
        .vector_size(1536)
        .table_name("swiftide_test")
        .build()?;

    let indexing_provider: Box<dyn SimplePrompt> =
        repository.config().indexing_provider().try_into()?;
    let embedding_provider: Box<dyn EmbeddingModel> =
        repository.config().embedding_provider().try_into()?;

    swiftide::indexing::Pipeline::from_loader(loader)
        .then_chunk(transformers::ChunkCode::try_for_language_and_chunk_size(
            repository.config().language,
            chunk_size,
        )?)
        .then(transformers::MetadataQACode::new(indexing_provider))
        .then_in_batch(transformers::Embed::new(embedding_provider))
        .then_store_with(lancedb)
        .run()
        .await
}
