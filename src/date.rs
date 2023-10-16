// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Debug;

use sqlx::types::time::OffsetDateTime;

#[derive(Clone)]
pub struct Date {
    pub time: OffsetDateTime,
}

impl Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Date")
            .field("time", &self.print_date())
            .finish()
    }
}

pub const MINUTE: i64 = 60;
pub const HOUR: i64 = MINUTE * 60;
pub const DAY: i64 = HOUR * 24;
pub const WEEK: i64 = DAY * 7;

impl Date {
    pub fn format(date: &OffsetDateTime) -> String {
        let timestamp = date.unix_timestamp();
        let now = OffsetDateTime::now_utc().unix_timestamp();

        let difference = now - timestamp;

        if difference >= 3 * WEEK {
            format!("{}{}{}", date.year(), date.month(), date.date())
        } else if (DAY..(3 * WEEK)).contains(&difference) {
            format!("{} days ago", date.hour())
        } else if (HOUR..DAY).contains(&difference) {
            format!("{} hours ago", date.hour())
        } else if (MINUTE..HOUR).contains(&difference) {
            format!("{} minutes ago", date.minute())
        } else {
            format!("{} seconds ago", date.second())
        }
    }

    /// print relative time from date
    pub fn print_date(&self) -> String {
        Self::format(&self.time)
    }

    /// print date
    pub fn date(&self) -> String {
        format!(
            "{}{}{}",
            self.time.year(),
            self.time.month(),
            self.time.date()
        )
    }

    pub fn new(unix: i64) -> Self {
        Self {
            time: OffsetDateTime::from_unix_timestamp(unix).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_date_test() {
        let mut n = Date {
            time: OffsetDateTime::now_utc(),
        };

        let timestamp = n.time.unix_timestamp();
        println!("timestamp: {}", timestamp);

        // seconds test
        assert!(n.print_date().contains("seconds ago"));
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - 5).unwrap();
        assert!(n.print_date().contains("seconds ago"));

        // minutes test
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - MINUTE * 2).unwrap();
        assert!(n.print_date().contains("minutes ago"));
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - MINUTE * 56).unwrap();
        assert!(n.print_date().contains("minutes ago"));

        // hours test
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - HOUR).unwrap();
        assert!(n.print_date().contains("hours ago"));
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - HOUR * 23).unwrap();
        assert!(n.print_date().contains("hours ago"));

        // days test
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - 2 * WEEK).unwrap();
        assert!(n.print_date().contains("days ago"));

        // date test
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - 6 * WEEK).unwrap();
        let date = format!("{}{}{}", n.time.year(), n.time.month(), n.time.date());
        assert!(n.print_date().contains(&date))
    }
}
