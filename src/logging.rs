use log::{info, LevelFilter};

pub fn init_logging(verbose: bool) {
    let mut level = LevelFilter::Error;
    if verbose {
        level = LevelFilter::Trace;
    }
    env_logger::builder().filter_level(level).init();
    info!("Initialized logging with log level {}", level)
}

