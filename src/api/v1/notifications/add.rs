// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::AppData;

use db_core::AddNotification;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddNotificationRequest {
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
    payload: web::Json<AddNotificationRequest>,
    data: AppData,
    id: Identity,
) -> ServiceResult<impl Responder> {
    let sender = id.identity().unwrap();
    // TODO handle error where payload.to doesn't exist

    let p = AddNotification {
        from: &sender,
        to: &payload.to,
        message: &payload.message,
        heading: &payload.heading,
    };

    data.db.create_notification(&p).await?;

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
    async fn notification_works_pg() {
        let data = pg::get_data().await;
        notification_works(data).await;
    }

    #[actix_rt::test]
    async fn notification_works_maria() {
        let data = maria::get_data().await;
        notification_works(data).await;
    }

    pub async fn notification_works(data: ArcData) {
        const NAME1: &str = "notifuser1";
        const NAME2: &str = "notiuser2";
        const PASSWORD: &str = "longpassworddomain";
        const EMAIL1: &str = "testnotification1@a.com";
        const EMAIL2: &str = "testnotification2@a.com";

        let data = &data;

        delete_user(data, NAME1).await;
        delete_user(data, NAME2).await;

        register_and_signin(data, NAME1, EMAIL1, PASSWORD).await;
        register_and_signin(data, NAME2, EMAIL2, PASSWORD).await;
        let (_creds, signin_resp) = signin(data, NAME1, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        let msg = AddNotificationRequest {
            to: NAME2.into(),
            heading: "Test notification".into(),
            message: "Testing notifications with a dummy message".into(),
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
