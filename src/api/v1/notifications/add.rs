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

use crate::errors::*;
use crate::Data;

#[derive(Serialize, Deserialize)]
pub struct AddNotification {
    pub to: String,
    pub heading: String,
    pub message: String,
}

/// route handler that adds a notification message
#[my_codegen::post(path="crate::V1_API_ROUTES.notifications.add", wrap="crate::CheckLogin")]
pub async fn add_notification(
    payload: web::Json<AddNotification>,
    data: web::Data<Data>,
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
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    use crate::*;

    #[actix_rt::test]
    async fn notification_works() {
        const NAME1: &str = "notifuser1";
        const NAME2: &str = "notiuser2";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL1: &str = "testnotification1@a.com";
        const EMAIL2: &str = "testnotification2@a.com";

        {
            let data = Data::new().await;
            delete_user(NAME1, &data).await;
            delete_user(NAME2, &data).await;
        }

        register_and_signin(NAME1, EMAIL1, PASSWORD).await;
        register_and_signin(NAME2, EMAIL2, PASSWORD).await;
        let (data, _creds, signin_resp) = signin(NAME1, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let mut app = get_app!(data).await;

        let msg = AddNotification {
            to: NAME2.into(),
            heading: "Test notification".into(),
            message: "Testeing notifications with a dummy message".into(),
        };

        let send_notification_resp = test::call_service(
            &mut app,
            post_request!(&msg, V1_API_ROUTES.notifications.add)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(send_notification_resp.status(), StatusCode::OK);
    }
}
