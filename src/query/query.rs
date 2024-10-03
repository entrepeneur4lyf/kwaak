use anyhow::Result;
use swiftide::{
    query::{self, answers, query_transformers, response_transformers},
    traits::{EmbeddingModel, SimplePrompt},
};

use crate::{repository::Repository, storage};

pub async fn query(repository: &Repository, query: &str) -> Result<String> {
    let query_provider: Box<dyn SimplePrompt> = repository.config().query_provider().try_into()?;
    let embedding_provider: Box<dyn EmbeddingModel> =
        repository.config().embedding_provider().try_into()?;

    let lancedb = storage::build_lancedb(repository)?;

    let pipeline = query::Pipeline::default()
        .then_transform_query(query_transformers::GenerateSubquestions::from_client(
            query_provider.clone(),
        ))
        .then_transform_query(query_transformers::Embed::from_client(
            embedding_provider.clone(),
        ))
        .then_retrieve(lancedb.build()?)
        .then_transform_response(response_transformers::Summary::from_client(
            query_provider.clone(),
        ))
        .then_answer(answers::Simple::from_client(query_provider.clone()));

    pipeline.query(query).await.map(|q| q.answer().to_string())
}
