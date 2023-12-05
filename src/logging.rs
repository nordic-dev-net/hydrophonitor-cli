use log::{info, LevelFilter};

pub fn init_logging(verbose: &String) {
    let level = match verbose.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Error //TODO throw an error
    };
    env_logger::builder().filter_level(level).init();
    info!("Initialized logging with log level {}", level)
}

