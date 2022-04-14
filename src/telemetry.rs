use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Compse multiple layers into a `tracing`'s subscriber.
///
/// Implementation notes:
///
/// `impl Subscriber` is used as return type to avoid need to
/// spell out the actual type returned by the subscriber.
/// We explicitly call out that the returned subscriber is
/// `Send` and `Sync`, making it possible to pass it to
/// `init_subscriber`.
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        std::io::stdout
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Filaed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}