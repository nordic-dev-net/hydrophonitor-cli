use log::{info, LevelFilter};

pub fn init_logging(verbose: usize) {
    let level = match verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        4 => LevelFilter::Trace,
        _ => LevelFilter::Error
    };
    env_logger::builder().filter_level(level).init();
    info!("Initialized logging with log level {}", level)
}

