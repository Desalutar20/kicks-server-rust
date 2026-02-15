use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

pub fn create_subscriber(
    pretty_log: bool,
    log_level: &str,
) -> (Box<dyn Subscriber + Sync + Send>, WorkerGuard) {
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or(EnvFilter::try_new(log_level).expect("Failed to create env filter"));

    match pretty_log {
        true => {
            let fmt_layer = fmt::layer::<Registry>()
                .with_writer(non_blocking)
                .with_target(false)
                .with_file(true)
                .with_line_number(true)
                .pretty();

            (
                Box::new(Registry::default().with(fmt_layer).with(env_filter)),
                guard,
            )
        }
        false => {
            let fmt_layer = fmt::layer::<Registry>()
                .with_writer(non_blocking)
                .with_target(false)
                .with_file(true)
                .with_line_number(true)
                .json();

            (
                Box::new(Registry::default().with(fmt_layer).with(env_filter)),
                guard,
            )
        }
    }
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    set_global_default(subscriber).expect("Failed to set subscriber");
}
