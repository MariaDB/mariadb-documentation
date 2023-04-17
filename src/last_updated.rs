use crate::Result;
use chrono::{Days, NaiveDate};
use core::fmt;
use std::{fs, io, path::Path};

pub enum FailedToReadLastUpdated {
    FileNotFound,
    InvalidFormat,
}

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

pub fn read_last_updated(archive_path: &Path) -> Result<NaiveDate, FailedToReadLastUpdated> {
    fs::read_to_string(archive_path.join("last_updated.txt"))
        .map_err(|_| FailedToReadLastUpdated::FileNotFound)?
        .parse()
        .map(|date: NaiveDate| date - Days::new(1))
        .map_err(|_| FailedToReadLastUpdated::InvalidFormat)
}

pub fn write_last_updated(archive_path: &Path) -> Result<(), io::Error> {
    let date = chrono::Local::now().date_naive();
    fs::write(archive_path.join("last_updated.txt"), date.to_string())
}
