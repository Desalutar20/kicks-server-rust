use kicksapi::{
    self,
    app::Application,
    configuration::Configuration,
    error::Result,
    telemetry::{create_subscriber, init_subscriber},
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Configuration::new();

    let (subscriber, _guard) = create_subscriber(
        config.application.pretty_log,
        config.application.log_level.as_str(),
    );
    init_subscriber(subscriber);

    let app = Application::build(&config).await?;
    let token = CancellationToken::new();

    app.run(token).await?;

    Ok(())
}
