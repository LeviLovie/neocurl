fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = neocurl::run() {
        tracing::error!("Error occured: {:?}", e);
        std::process::exit(1);
    }
}
