/*
 * mCaptcha is a PoW based DoS protection software.
 * This is the frontend web component of the mCaptcha system
 * Copyright © 2023 Aravinth Manivnanan <realaravinth@batsense.net>.
 *
 * Use of this source code is governed by Apache 2.0 or MIT license.
 * You shoud have received a copy of MIT and Apache 2.0 along with
 * this program. If not, see <https://spdx.org/licenses/MIT.html> for
 * MIT or <http://www.apache.org/licenses/LICENSE-2.0> for Apache.
 */

import log from "../logger";

import prove from "./prove";
import {PoWConfig, ServiceWorkerWork} from "./types";

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
