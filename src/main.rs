#[allow(unused_imports)]
use tracing::{span, Level, event, instrument, debug, trace, warn, info};

fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
	.with_ansi(false) // disable colors
	.with_target(true)
	.with_max_level(Level::TRACE)
	.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
	.finish();

    match tracing::subscriber::set_global_default(subscriber) {
	Ok(_) => event!(target: "setup_subscriber", Level::TRACE, "created tracing subscriber"),
	Err(error) => event!(target:"setup_subscriber", Level::ERROR, "could not create tracing subscriber, error = {:?}", error)
    }
}

fn main() {
    setup_tracing();
    debug!(target: "main", "start");
    debug!(target: "main", "finish");
}
