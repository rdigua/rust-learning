// Converts a date given by NaiveDate::from_ymd and NaiveTime::from_hms to UNIX timestamp using NaiveDateTime::timestamp. Then it calculates what was the date after one billion seconds since January 1, 1970 0:00:00 UTC, using NaiveDateTime::from_timestamp.
extern crate chrono;

use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time, date_time.timestamp());

    let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
        date_time_after_a_billion_seconds);
}

