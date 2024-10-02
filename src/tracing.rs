use crate::repository::Repository;
use anyhow::Result;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

// For now just log to stdout and file
pub fn init(repository: &Repository) -> Result<()> {
    let log_dir = repository.config().log_dir();

    let file_appender = tracing_appender::rolling::never(
        log_dir,
        format!("{}.log", repository.config().project_name),
    );

    let env_filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("kwaak=info,swiftide=info,error"))?;

    let fmt_layer = fmt::layer().with_writer(file_appender);

    tracing_subscriber::registry()
        .with(env_filter_layer)
        .with(fmt_layer)
        .try_init()?;

    Ok(())
}
