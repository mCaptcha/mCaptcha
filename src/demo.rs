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
use std::time::Duration;
//use std::sync::atomicBool

use actix::clock::sleep;
use actix::spawn;
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
    handle: JoinHandle<()>,
}

impl DemoUser {
    pub async fn spawn(data: AppData, duration: Duration) -> ServiceResult<Self> {
        let handle = Self::run(data, duration).await?;
        let d = Self { handle };

        Ok(d)
    }

    #[allow(dead_code)]
    pub fn abort(&self) {
        self.handle.abort();
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
        duration: Duration,
    ) -> ServiceResult<JoinHandle<()>> {
        Self::register_demo_user(&data).await?;

        let fut = async move {
            loop {
                sleep(duration).await;
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
        let user = DemoUser::spawn(data, duration).await.unwrap();
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
        user.abort();
    }
}
