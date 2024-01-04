// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::Duration;
//use std::sync::atomicBool

use actix::clock::sleep;
use actix::spawn;
use tokio::sync::oneshot::{channel, error::TryRecvError, Receiver, Sender};
use tokio::task::JoinHandle;

use crate::api::v1::account::delete::runners::delete_user;
use crate::api::v1::account::{username::runners::username_exists, AccountCheckPayload};
use crate::api::v1::auth::runners::{register_runner, Register};
use crate::*;

use errors::*;

/// Demo username
pub const DEMO_USER: &str = "aaronsw";
/// Demo password
pub const DEMO_PASSWORD: &str = "password";

pub struct DemoUser {
    tx: Sender<()>,
}

impl DemoUser {
    pub async fn spawn(data: AppData, duration: u32) -> ServiceResult<(Self, JoinHandle<()>)> {
        let (tx, rx) = channel();
        let handle = Self::run(data, duration, rx).await?;
        let d = Self { tx };

        Ok((d, handle))
    }

    #[allow(dead_code)]
    pub fn abort(mut self) {
        self.tx.send(());
    }

    /// register demo user runner
    async fn register_demo_user(data: &AppData) -> ServiceResult<()> {
        let user_exists_payload = AccountCheckPayload {
            val: DEMO_USER.into(),
        };

        if !username_exists(&user_exists_payload, data).await?.exists {
            let register_payload = Register {
                username: DEMO_USER.into(),
                password: DEMO_PASSWORD.into(),
                confirm_password: DEMO_PASSWORD.into(),
                email: None,
            };

            log::info!("Registering demo user");
            match register_runner(&register_payload, data).await {
                Err(ServiceError::UsernameTaken) | Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    async fn delete_demo_user(data: &AppData) -> ServiceResult<()> {
        log::info!("Deleting demo user");
        delete_user(DEMO_USER, data).await?;
        Ok(())
    }

    pub async fn run(
        data: AppData,
        duration: u32,
        mut rx: Receiver<()>,
    ) -> ServiceResult<JoinHandle<()>> {
        Self::register_demo_user(&data).await?;

        fn can_run(rx: &mut Receiver<()>) -> bool {
            match rx.try_recv() {
                Err(TryRecvError::Empty) => true,
                _ => false,
            }
        }

        let mut exit = false;
        let fut = async move {
            loop {
                if exit {
                    break;
                }
                for _ in 0..duration {
                    if can_run(&mut rx) {
                        sleep(Duration::new(1, 0)).await;
                        continue;
                    } else {
                        exit = true;
                        break;
                    }
                }

                if let Err(e) = Self::delete_demo_user(&data).await {
                    log::error!("Error while deleting demo user: {:?}", e);
                }

                if let Err(e) = Self::register_demo_user(&data).await {
                    log::error!("Error while registering demo user: {:?}", e);
                }

            }
        };
        let handle = spawn(fut);
        Ok(handle)
    }
}

#[cfg(test)]
mod tests {

    use actix_web::test;
    use libmcaptcha::defense::Level;

    use super::*;
    use crate::tests::*;

    const DURATION: u64 = 25;

    #[actix_rt::test]
    async fn demo_account_works_pg() {
        let data = crate::tests::pg::get_data().await;
        demo_account_works(data).await;
    }

    #[actix_rt::test]
    async fn demo_account_works_maria() {
        let data = crate::tests::maria::get_data().await;
        demo_account_works(data).await;
    }

    async fn demo_account_works(data_inner: ArcData) {
        let data_inner = &data_inner;
        let data = AppData::new(data_inner.clone());
        crate::tests::delete_user(data_inner, DEMO_USER).await;
        let duration = Duration::from_secs(DURATION);

        // register works
        DemoUser::register_demo_user(&data).await.unwrap();
        let payload = AccountCheckPayload {
            val: DEMO_USER.into(),
        };
        assert!(username_exists(&payload, &data).await.unwrap().exists);
        signin(data_inner, DEMO_USER, DEMO_PASSWORD).await;

        // deletion works
        assert!(DemoUser::delete_demo_user(&data).await.is_ok());
        assert!(!username_exists(&payload, &data).await.unwrap().exists);

        // test the runner
        let user = DemoUser::spawn(data, DURATION as u32).await.unwrap();
        let (_, signin_resp, token_key) =
            add_levels_util(data_inner, DEMO_USER, DEMO_PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data_inner).await;

        let resp = test::call_service(
            &app,
            post_request!(&token_key, crate::V1_API_ROUTES.captcha.get)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let res_levels: Vec<Level> = test::read_body_json(resp).await;
        assert!(!res_levels.is_empty());

        sleep(Duration::from_secs(DURATION * 2)).await;

        let resp = test::call_service(
            &app,
            post_request!(&token_key, crate::V1_API_ROUTES.captcha.get)
                .cookie(cookies)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let res_levels: Vec<Level> = test::read_body_json(resp).await;
        assert!(res_levels.is_empty());
        user.0.abort();
        user.1.await.unwrap();
    }
}
