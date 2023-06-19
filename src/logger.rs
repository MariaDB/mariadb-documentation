use log::LevelFilter;
use simplelog::{
    CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};
use std::{fs::File, path::Path};

pub fn init(verbose: bool, file_output: Option<&Path>) {
    let mut config = ConfigBuilder::new();
    let _ = config.set_time_offset_to_local();
    let config = config.build();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![];

    loggers.push(TermLogger::new(
        if verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Error
        },
        config.clone(),
        TerminalMode::Stdout,
        simplelog::ColorChoice::Auto,
    ));

    if let Some(path) = file_output {
        loggers.push(WriteLogger::new(
            LevelFilter::Info,
            config,
            File::create(path).unwrap_or_else(|err| panic!("{err} - Failed to write to {path:?}")),
        ));
    }
    CombinedLogger::init(loggers).unwrap();
}
