use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[allow(dead_code)]
pub fn init() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}
