// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::sleep;

use crate::errors::*;
use crate::settings::Settings;
use crate::AppData;
use crate::V1_API_ROUTES;

#[async_trait::async_trait]
pub trait SurveyClientTrait {
    async fn start_job(&self) -> ServiceResult<(oneshot::Sender<()>, JoinHandle<()>)>;
    async fn schedule_upload_job(&self) -> ServiceResult<()>;
    async fn is_online(&self) -> ServiceResult<bool>;
    async fn register(&self) -> ServiceResult<()>;
}

#[derive(Clone, Debug, Default)]
pub struct SecretsStore {
    store: Arc<RwLock<HashMap<String, String>>>,
}

impl SecretsStore {
    pub fn get(&self, key: &str) -> Option<String> {
        let r = self.store.read().unwrap();
        r.get(key).map(|x| x.to_owned())
    }

    pub fn rm(&self, key: &str) {
        let mut w = self.store.write().unwrap();
        w.remove(key);
        drop(w);
    }

    pub fn set(&self, key: String, value: String) {
        let mut w = self.store.write().unwrap();
        w.insert(key, value);
        drop(w);
    }
}

#[derive(Clone)]
pub struct Survey {
    client: Client,
    app_ctx: AppData,
}
impl Survey {
    pub fn new(app_ctx: AppData) -> Self {
        if app_ctx.settings.survey.is_none() {
            panic!("Survey uploader shouldn't be initialized it isn't configured, please report this bug")
        }
        Survey {
            client: Client::new(),
            app_ctx,
        }
    }
}

#[async_trait::async_trait]
impl SurveyClientTrait for Survey {
    async fn start_job(&self) -> ServiceResult<(oneshot::Sender<()>, JoinHandle<()>)> {
        fn can_run(rx: &mut oneshot::Receiver<()>) -> bool {
            match rx.try_recv() {
                Err(oneshot::error::TryRecvError::Empty) => true,
                _ => false,
            }
        }

        let (tx, mut rx) = oneshot::channel();
        let this = self.clone();
        let mut register = false;
        let fut = async move {
            loop {
                if !can_run(&mut rx) {
                    log::info!("Stopping survey uploads");
                    break;
                }

                if !register {
                    loop {
                        if this.is_online().await.unwrap() {
                            this.register().await.unwrap();
                            register = true;
                            break;
                        } else {
                            sleep(Duration::new(1, 0)).await;
                        }
                    }
                }

                for i in 0..this.app_ctx.settings.survey.as_ref().unwrap().rate_limit {
                    if !can_run(&mut rx) {
                        log::info!("Stopping survey uploads");
                        break;
                    }
                    sleep(Duration::new(1, 0)).await;
                }
                let _ = this.schedule_upload_job().await;

                // for url in this.app_ctx.settings.survey.as_ref().unwrap().nodes.iter() {
                //     if !can_run(&mut rx) {
                //         log::info!("Stopping survey uploads");
                //         break;
                //     }
                //     log::info!("Uploading to survey instance {}", url);
                // }
            }
        };
        let handle = tokio::spawn(fut);
        Ok((tx, handle))
    }
    async fn is_online(&self) -> ServiceResult<bool> {
        let res = self
            .client
            .get(format!(
                "http://{}{}",
                self.app_ctx.settings.server.get_ip(),
                V1_API_ROUTES.meta.health
            ))
            .send()
            .await
            .unwrap();
        Ok(res.status() == 200)
    }

    async fn schedule_upload_job(&self) -> ServiceResult<()> {
        log::debug!("Running upload job");
        #[derive(Serialize)]
        struct Secret {
            secret: String,
        }
        let mut page = 0;
        loop {
            let psuedo_ids = self.app_ctx.db.analytics_get_all_psuedo_ids(page).await?;
            if psuedo_ids.is_empty() {
                log::debug!("upload job complete, no more IDs to upload");
                break;
            }
            for id in psuedo_ids {
                for url in self.app_ctx.settings.survey.as_ref().unwrap().nodes.iter() {
                    if let Some(secret) = self.app_ctx.survey_secrets.get(url.as_str()) {
                        let payload = Secret { secret };

                        log::info!("Uploading to survey instance {} campaign {id}", url);
                        let mut url = url.clone();
                        url.set_path(&format!("/mcaptcha/api/v1/{id}/upload"));
                        let resp =
                            self.client.post(url).json(&payload).send().await.unwrap();
                        println!("{}", resp.text().await.unwrap());
                    }
                }
            }
            page += 1;
        }
        Ok(())
    }

    async fn register(&self) -> ServiceResult<()> {
        #[derive(Serialize)]
        struct MCaptchaInstance {
            url: url::Url,
            auth_token: String,
        }

        let this_instance_url = self
            .app_ctx
            .settings
            .survey
            .as_ref()
            .unwrap()
            .instance_root_url
            .clone();
        for url in self.app_ctx.settings.survey.as_ref().unwrap().nodes.iter() {
            // mCaptcha/survey must send this token while uploading secret to authenticate itself
            // this token must be sent to mCaptcha/survey with the registration payload
            let secret_upload_auth_token = crate::api::v1::mcaptcha::get_random(20);

            let payload = MCaptchaInstance {
                url: this_instance_url.clone(),
                auth_token: secret_upload_auth_token.clone(),
            };

            // SecretsStore will store auth tokens generated by both mCaptcha/mCaptcha and
            // mCaptcha/survey
            //
            // Storage schema:
            // - mCaptcha/mCaptcha generated auth token: (<auth_token>, <survey_instance_url>)
            // - mCaptcha/survey generated auth token (<survey_instance_url>, <auth_token)
            self.app_ctx
                .survey_secrets
                .set(secret_upload_auth_token, url.to_string());
            let mut url = url.clone();
            url.set_path("/mcaptcha/api/v1/register");
            let resp = self.client.post(url).json(&payload).send().await.unwrap();
        }
        Ok(())
    }
}
