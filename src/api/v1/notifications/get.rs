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

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::errors::*;
use crate::Data;

pub struct Notification {
    pub name: String,
    pub heading: String,
    pub message: String,
    pub received: OffsetDateTime,
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct NotificationResp {
    pub name: String,
    pub heading: String,
    pub message: String,
    pub received: i64,
    pub id: i32,
}

impl From<Notification> for NotificationResp {
    fn from(n: Notification) -> Self {
        NotificationResp {
            name: n.name,
            heading: n.heading,
            received: n.received.unix_timestamp(),
            id: n.id,
            message: n.message,
        }
    }
}
/// route handler that gets all unread notifications
pub async fn get_notification(
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let receiver = id.identity().unwrap();
    // TODO handle error where payload.to doesnt exist

    let mut notifications = sqlx::query_file_as!(
        Notification,
        "src/api/v1/notifications/get_all_unread.sql",
        &receiver
    )
    .fetch_all(&data.db)
    .await?;

    let resp: Vec<NotificationResp> = notifications
        .drain(0..)
        .map(|x| {
            let y: NotificationResp = x.into();
            y
        })
        .collect();

    Ok(HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;

    use super::*;
    use crate::api::v1::notifications::add::AddNotification;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn notification_get_works() {
        const NAME1: &str = "notifuser12";
        const NAME2: &str = "notiuser22";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL1: &str = "testnotification12@a.com";
        const EMAIL2: &str = "testnotification22@a.com";
        const HEADING: &str = "testing notifications get";
        const MESSAGE: &str = "testing notifications get message";

        {
            let data = Data::new().await;
            delete_user(NAME1, &data).await;
            delete_user(NAME2, &data).await;
        }

        register_and_signin(NAME1, EMAIL1, PASSWORD).await;
        register_and_signin(NAME2, EMAIL2, PASSWORD).await;
        let (data, _creds, signin_resp) = signin(NAME1, PASSWORD).await;
        let (_data, _creds2, signin_resp2) = signin(NAME2, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let cookies2 = get_cookie!(signin_resp2);
        let mut app = get_app!(data).await;

        let msg = AddNotification {
            to: NAME2.into(),
            heading: HEADING.into(),
            message: MESSAGE.into(),
        };

        let send_notification_resp = test::call_service(
            &mut app,
            post_request!(&msg, V1_API_ROUTES.notifications.add)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(send_notification_resp.status(), StatusCode::OK);

        let get_notifications_resp = test::call_service(
            &mut app,
            test::TestRequest::get()
                .uri(V1_API_ROUTES.notifications.get)
                .cookie(cookies2.clone())
                .to_request(),
        )
        .await;
        assert_eq!(get_notifications_resp.status(), StatusCode::OK);

        let mut notifications: Vec<NotificationResp> =
            test::read_body_json(get_notifications_resp).await;
        let notification = notifications.pop().unwrap();
        assert_eq!(notification.name, NAME1);
        assert_eq!(notification.message, MESSAGE);
        assert_eq!(notification.heading, HEADING);
    }
}
