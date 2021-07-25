/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use sqlx::types::time::OffsetDateTime;

pub struct Date {
    pub time: OffsetDateTime,
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
            date.format("%d-%m-%y")
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

    pub fn print_date(&self) -> String {
        Self::format(&self.time)
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
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - 5);
        assert!(n.print_date().contains("seconds ago"));

        // minutes test
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - MINUTE * 2);
        assert!(n.print_date().contains("minutes ago"));
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - MINUTE * 56);
        assert!(n.print_date().contains("minutes ago"));

        // hours test
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - HOUR);
        assert!(n.print_date().contains("hours ago"));
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - HOUR * 23);
        assert!(n.print_date().contains("hours ago"));

        // days test
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - 2 * WEEK);
        assert!(n.print_date().contains("days ago"));

        // date test
        n.time = OffsetDateTime::from_unix_timestamp(timestamp - 6 * WEEK);
        let date = n.time.format("%d-%m-%y");
        assert!(n.print_date().contains(&date))
    }
}
