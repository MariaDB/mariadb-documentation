use crate::Result;
use chrono::NaiveDate;
use std::{fs, io};

const LAST_UPDATED_PATH: &str = "last_updated.txt";

pub fn read_last_updated() -> NaiveDate {
    fs::read_to_string(LAST_UPDATED_PATH)
        .map(|date| date.parse().expect("Invalid last_updated.txt"))
        .unwrap_or(NaiveDate::MIN)
}

pub fn write_last_updated() -> Result<(), io::Error> {
    let date = chrono::Local::now().date_naive();
    fs::write(LAST_UPDATED_PATH, date.to_string())
}
