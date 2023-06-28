use crate::Result;
use chrono::{Days, NaiveDateTime};
use core::fmt;
use std::{fs, io, path::Path};

pub enum FailedToReadLastUpdated {
    FileNotFound,
    InvalidFormat,
}

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

impl fmt::Display for FailedToReadLastUpdated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileNotFound => write!(
                f,
                "Could not find 'last_updated.txt' in the mariadb_kb_archive.\n \
                    Run the standard scrape to update."
            ),
            Self::InvalidFormat => write!(f, "Invalid date format in 'last_updated.txt'."),
        }
    }
}

pub fn read_last_updated(archive_path: &Path) -> Result<NaiveDateTime, FailedToReadLastUpdated> {
    fs::read_to_string(archive_path.join("last_updated.txt"))
        .map(|str| NaiveDateTime::parse_from_str(&str, DATE_FORMAT))
        .map_err(|_| FailedToReadLastUpdated::FileNotFound)?
        .map(|date: NaiveDateTime| date - Days::new(1))
        .map_err(|_| FailedToReadLastUpdated::InvalidFormat)
}

pub fn write_last_updated(archive_path: &Path) -> Result<(), io::Error> {
    let date = chrono::Local::now().naive_local().format(DATE_FORMAT);
    fs::write(archive_path.join("last_updated.txt"), date.to_string())
}
