/*
 * mCaptcha is a PoW based DoS protection software.
 * This is the frontend web component of the mCaptcha system
 * Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
 *
 * Use of this source code is governed by Apache 2.0 or MIT license.
 * You shoud have received a copy of MIT and Apache 2.0 along with
 * this program. If not, see <https://spdx.org/licenses/MIT.html> for
 * MIT or <http://www.apache.org/licenses/LICENSE-2.0> for Apache.
 */

import { Work, ServiceWorkerWork } from "./types";
import fetchPoWConfig from "./fetchPoWConfig";
import sendWork from "./sendWork";
import sendToParent from "./sendToParent";
import * as CONST from "./const";

import "./main.scss";

let LOCK = false;
const worker = new Worker("/bench.js");

/** add  mcaptcha widget element to DOM */
export const registerVerificationEventHandler = (): void => {
  const verificationContainer = <HTMLElement>(
    document.querySelector(".widget__verification-container")
  );
  verificationContainer.style.display = "flex";
  CONST.btn().addEventListener("click", (e) => solveCaptchaRunner(e));
};

export const solveCaptchaRunner = async (e: Event): Promise<void> => {
  if (LOCK) {
    e.preventDefault();
    return;
  }

  try {
    LOCK = true;
    if (CONST.btn().checked == false) {
      CONST.messageText().before();
      LOCK = false;
      return;
    }
    e.preventDefault();
    // steps:

    // 1. show during
    CONST.messageText().during();
    // 1. get config
    const config = await fetchPoWConfig();
    // 2. prove work
    worker.postMessage(config);

    worker.onmessage = async (event: MessageEvent) => {
      const resp: ServiceWorkerWork = event.data;
      console.log(
        `Proof generated. Difficuly: ${config.difficulty_factor} Duration: ${resp.duration}`
      );

      const proof: Work = {
        key: CONST.sitekey(),
        string: config.string,
        nonce: resp.work.nonce,
        result: resp.work.result,
      };

      // 3. submit work
      const token = await sendWork(proof);
      // 4. send token
      sendToParent(token);
      // 5. mark checkbox checked
      CONST.btn().checked = true;
      CONST.messageText().after();
      LOCK = false;
    };
  } catch (e) {
    CONST.messageText().error();
    console.error(e);
    LOCK = false;
  }
};

registerVerificationEventHandler();
