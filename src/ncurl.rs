use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let filter = tracing_subscriber::filter::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::filter::EnvFilter::new("warn"));
    let registry = tracing_subscriber::registry::Registry::default()
        .with(filter)
        .with(tracing_subscriber::fmt::layer());
    tracing::subscriber::set_global_default(registry).expect("Failed to set global subscriber");

    if let Err(e) = neocurl::run() {
        tracing::error!("Error occured: {:?}", e);
        std::process::exit(1);
    }
}
