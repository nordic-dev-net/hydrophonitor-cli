use log::{error, info, LevelFilter};

pub fn init_logging(verbose: &String) {
    let mut invalid_verbosity = false;
    let level = match verbose.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => {
            invalid_verbosity = true;
            LevelFilter::Error
        }
    };
    env_logger::builder().filter_level(level).init();
    if invalid_verbosity {
        error!("Invalid verbosity {verbose}! Using log level 'error' as fallback!")
    } else {
        info!("Initialized logging with log level {}", level)
    }
}

