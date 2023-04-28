use log::LevelFilter;
use simplelog::{
    CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};
use std::{fs::File, path::Path};

pub fn init(stdout: bool, file_output: Option<&Path>) {
    let mut config = ConfigBuilder::new();
    let _ = config.set_time_offset_to_local();
    let config = config.build();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![];
    if let Some(path) = file_output {
        loggers.push(WriteLogger::new(
            LevelFilter::Info,
            config.clone(),
            File::create(path).unwrap_or_else(|err| panic!("{err} - Failed to write to {path:?}")),
        ));
    }

    if stdout {
        loggers.push(TermLogger::new(
            LevelFilter::Info,
            config,
            TerminalMode::Stdout,
            simplelog::ColorChoice::Auto,
        ));
    }

    CombinedLogger::init(loggers).unwrap();
}
