// Copyright (C) 2024// Copyright (C) 2024  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::Duration;
//use std::sync::atomicBool

use actix::clock::sleep;
use actix::spawn;
use tokio::sync::oneshot::{channel, error::TryRecvError, Receiver, Sender};
use tokio::task::JoinHandle;

use crate::api::v1::mcaptcha::easy::{
    update_runner, TrafficPatternRequest, UpdateTrafficPattern,
};
use crate::*;

use errors::*;

pub struct UpdateEasyCaptcha {
    tx: Sender<()>,
}

impl UpdateEasyCaptcha {
    pub async fn spawn(
        data: AppData,
        duration: u32,
    ) -> ServiceResult<(Self, JoinHandle<()>)> {
        let (tx, rx) = channel();
        let handle = Self::run(data, duration, rx).await?;
        let d = Self { tx };

        Ok((d, handle))
    }

    #[allow(dead_code)]
    pub fn abort(mut self) {
        self.tx.send(());
    }

    /// update configurations
    async fn update_captcha_configurations(
        data: &AppData,
        rx: &mut Receiver<()>,
    ) -> ServiceResult<()> {
        let limit = 10;
        let mut offset = 0;
        let mut page = 0;
        loop {
            offset = page * limit;

            if !Self::can_run(rx) {
                return Ok(());
            }

            let mut patterns = data.db.get_all_easy_captchas(limit, offset).await?;
            if patterns.is_empty() {
                break;
            }
            for pattern in patterns.drain(0..) {
                if !Self::can_run(rx) {
                    return Ok(());
                }

                let publish_benchmarks =
                    data.db.analytics_captcha_is_published(&pattern.key).await?;

                let req = UpdateTrafficPattern {
                    pattern: TrafficPatternRequest {
                        avg_traffic: pattern.traffic_pattern.avg_traffic,
                        peak_sustainable_traffic: pattern
                            .traffic_pattern
                            .peak_sustainable_traffic,
                        broke_my_site_traffic: pattern
                            .traffic_pattern
                            .broke_my_site_traffic,
                        description: pattern.description,
                        publish_benchmarks,
                    },
                    key: pattern.key,
                };
                if !Self::can_run(rx) {
                    return Ok(());
                }

                update_runner(&data, req, pattern.username).await?;
            }
            page += 1;
        }
        Ok(())
    }

    fn can_run(rx: &mut Receiver<()>) -> bool {
        match rx.try_recv() {
            Err(TryRecvError::Empty) => true,
            _ => false,
        }
    }

    pub async fn run(
        data: AppData,
        duration: u32,
        mut rx: Receiver<()>,
    ) -> ServiceResult<JoinHandle<()>> {
        let mut exit = false;
        let fut = async move {
            loop {
                if exit {
                    break;
                }
                for _ in 0..duration {
                    if Self::can_run(&mut rx) {
                        sleep(Duration::new(1, 0)).await;
                        continue;
                    } else {
                        exit = true;
                        break;
                    }
                }

                if let Some(err) = Self::update_captcha_configurations(&data, &mut rx)
                    .await
                    .err()
                {
                    log::error!(
                        "Tried to update easy captcha configurations in background {:?}",
                        err
                    );
                }
            }
        };
        let handle = spawn(fut);
        Ok(handle)
    }
}
