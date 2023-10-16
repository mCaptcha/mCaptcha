// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use sailfish::TemplateOnce;
use sqlx::types::time::OffsetDateTime;

use crate::date::Date;
use crate::errors::PageResult;
use crate::AppData;

#[derive(TemplateOnce)]
#[template(path = "panel/notifications/index.html")]
pub struct IndexPage {
    /// notifications
    n: Vec<Notification>,
}

impl IndexPage {
    fn new(n: Vec<Notification>) -> Self {
        IndexPage { n }
    }
}

pub struct Notification {
    pub name: String,
    pub heading: String,
    pub message: String,
    pub received: OffsetDateTime,
    pub id: i32,
}

impl From<db_core::Notification> for Notification {
    fn from(n: db_core::Notification) -> Self {
        Notification {
            name: n.name.unwrap(),
            heading: n.heading.unwrap(),
            received: OffsetDateTime::from_unix_timestamp(n.received.unwrap()).unwrap(),
            id: n.id.unwrap(),
            message: n.message.unwrap(),
        }
    }
}

impl Notification {
    pub fn print_date(&self) -> String {
        Date::format(&self.received)
    }
}

const PAGE: &str = "Notifications";

#[my_codegen::get(
    path = "crate::PAGES.panel.notifications",
    wrap = "crate::pages::get_middleware()"
)]
pub async fn notifications(data: AppData, id: Identity) -> PageResult<impl Responder> {
    let receiver = id.identity().unwrap();
    // TODO handle error where payload.to doesn't exist

    //    let mut notifications = runner::get_notification(&data, &receiver).await?;
    let mut notifications = data.db.get_all_unread_notifications(&receiver).await?;
    let notifications = notifications.drain(0..).map(|x| x.into()).collect();

    let body = IndexPage::new(notifications).render_once().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::*;

    #[test]
    fn print_date_test() {
        let mut n = Notification {
            received: OffsetDateTime::now_utc(),
            name: String::default(),
            heading: String::default(),
            message: String::default(),
            id: 1,
        };

        let timestamp = n.received.unix_timestamp();
        println!("timestamp: {}", timestamp);

        // seconds test
        assert!(n.print_date().contains("seconds ago"));
        n.received = OffsetDateTime::from_unix_timestamp(timestamp - 5).unwrap();
        assert!(n.print_date().contains("seconds ago"));

        // minutes test
        n.received =
            OffsetDateTime::from_unix_timestamp(timestamp - MINUTE * 2).unwrap();
        assert!(n.print_date().contains("minutes ago"));
        n.received =
            OffsetDateTime::from_unix_timestamp(timestamp - MINUTE * 56).unwrap();
        assert!(n.print_date().contains("minutes ago"));

        // hours test
        n.received = OffsetDateTime::from_unix_timestamp(timestamp - HOUR).unwrap();
        assert!(n.print_date().contains("hours ago"));
        n.received = OffsetDateTime::from_unix_timestamp(timestamp - HOUR * 23).unwrap();
        assert!(n.print_date().contains("hours ago"));

        // days test
        n.received = OffsetDateTime::from_unix_timestamp(timestamp - 2 * WEEK).unwrap();
        assert!(n.print_date().contains("days ago"));

        // date test
        n.received = OffsetDateTime::from_unix_timestamp(timestamp - 6 * WEEK).unwrap();
        let date = format!(
            "{}{}{}",
            n.received.year(),
            n.received.month(),
            n.received.date()
        );
        assert!(n.print_date().contains(&date))
    }
}
