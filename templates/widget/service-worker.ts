// Copyright © 2023 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import log from "../logger";

import prove from "./prove";
import { PoWConfig, ServiceWorkerWork } from "./types";

log.log("worker registered");
onmessage = async (e) => {
  console.debug("message received at worker");
  const config: PoWConfig = e.data;

  const work = await prove(config);
  const res: ServiceWorkerWork = {
    work,
  };

  postMessage(res);
};
