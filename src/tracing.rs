use crate::repository::Repository;
use anyhow::Result;
use log::LevelFilter;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

// For now just log to stdout and file
pub fn init(repository: &Repository) -> Result<()> {
    let log_dir = repository.config().log_dir();

    let file_appender = tracing_appender::rolling::never(
        log_dir,
        format!("{}.log", repository.config().project_name),
    );

    let fmt_layer = fmt::layer().with_writer(file_appender);

    // Logs the file layer will capture
    let env_filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("kwaak=info,swiftide=info,error"))?;

    let tui_layer = tui_logger::tracing_subscriber_layer();

    // The log level tui logger will capture
    let default_level = if cfg!(debug_assertions) {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    tui_logger::init_logger(default_level)?;
    tui_logger::set_default_level(default_level);

    let mut layers = vec![
        tui_layer.boxed(),
        env_filter_layer.boxed(),
        fmt_layer.boxed(),
    ];

    if cfg!(feature = "otel") {
        let provider = otel_provider();
        let tracer = provider.tracer("readme_example");

        // Create a tracing layer with the configured tracer
        let layer = OpenTelemetryLayer::new(tracer);

        layers.push(layer.boxed());
    }

    let registry = tracing_subscriber::registry().with(layers);
    registry.try_init()?;

    Ok(())
}

#[cfg(feature = "otel")]
fn otel_provider() -> TracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("failed to create otlp exporter");

    TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build()
}
