use log::LevelFilter;
use simplelog::{ConfigBuilder, WriteLogger};
use std::fs::File;

pub fn init() {
    let mut config = ConfigBuilder::new();
    let _ = config.set_time_offset_to_local();

    WriteLogger::init(
        LevelFilter::Info,
        config.build(),
        File::create("log.log").unwrap(),
    )
    .unwrap();
}
