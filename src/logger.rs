use log::LevelFilter;
use simplelog::{CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, WriteLogger};
use std::fs::File;

pub fn init(verbose: bool) {
    let mut config = ConfigBuilder::new();
    let _ = config.set_time_offset_to_local();
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![WriteLogger::new(
        LevelFilter::Info,
        config.build(),
        File::create("log.log").unwrap(),
    )];
    if verbose {
        loggers.push(TermLogger::new(
            LevelFilter::Info,
            config.build(),
            simplelog::TerminalMode::Stdout,
            simplelog::ColorChoice::Always,
        ));
    }

    CombinedLogger::init(loggers).unwrap();
}
