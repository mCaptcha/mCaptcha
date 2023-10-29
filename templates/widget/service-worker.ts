// Copyright © 2023 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import log from "../logger";

import prove from "./prove";
import { PoWConfig, ServiceWorkerMessage, ServiceWorkerWork } from "./types";

log.log("worker registered");
onmessage = async (e) => {
  console.debug("message received at worker");
  const config: PoWConfig = e.data;

  const progressCallback = (nonce: number) => {
    const res: ServiceWorkerMessage = {
      type: "progress",
      nonce: nonce,
    };

    postMessage(res);
  };

  const work = await prove(config, progressCallback);
  const w: ServiceWorkerWork = {
    work,
  };

  const res: ServiceWorkerMessage = {
    type: "work",
    value: w,
  };

  postMessage(res);
};
