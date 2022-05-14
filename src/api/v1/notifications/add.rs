/*
* Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

use crate::errors::*;
use crate::AppData;

#[derive(Serialize, Deserialize)]
pub struct AddNotification {
    pub to: String,
    pub heading: String,
    pub message: String,
}

/// route handler that adds a notification message
#[my_codegen::post(
    path = "crate::V1_API_ROUTES.notifications.add",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn add_notification(
    payload: web::Json<AddNotification>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let sender = id.identity().unwrap();
    // TODO handle error where payload.to doesnt exist
    sqlx::query!(
        "INSERT INTO mcaptcha_notifications (
              heading, message, tx, rx)
              VALUES  (
              $1, $2,
                  (SELECT ID FROM mcaptcha_users WHERE name = $3),
                  (SELECT ID FROM mcaptcha_users WHERE name = $4)
                      );",
        &payload.heading,
        &payload.message,
        &sender,
        &payload.to,
    )
    .execute(&data.db)
    .await?;
    Ok(HttpResponse::Ok())
}

#[cfg(test)]
pub mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    pub async fn notification_works() {
        const NAME1: &str = "notifuser1";
        const NAME2: &str = "notiuser2";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL1: &str = "testnotification1@a.com";
        const EMAIL2: &str = "testnotification2@a.com";

        let data = crate::data::Data::new().await;
        let data = &data;

        delete_user(data, NAME1).await;
        delete_user(data, NAME2).await;

        register_and_signin(data, NAME1, EMAIL1, PASSWORD).await;
        register_and_signin(data, NAME2, EMAIL2, PASSWORD).await;
        let (_creds, signin_resp) = signin(data, NAME1, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        let msg = AddNotification {
            to: NAME2.into(),
            heading: "Test notification".into(),
            message: "Testeing notifications with a dummy message".into(),
        };

        let send_notification_resp = test::call_service(
            &app,
            post_request!(&msg, V1_API_ROUTES.notifications.add)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(send_notification_resp.status(), StatusCode::OK);
    }
}
