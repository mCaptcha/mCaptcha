// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import { Work, ServiceWorkerMessage } from "./types";
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
  const PROGRESS_FILL = <HTMLElement>document.querySelector(".progress__fill");

  const setWidth = (width: number) => {
    PROGRESS_FILL.style.width = `${width}%`;
    PROGRESS_FILL.ariaValueNow = <any>parseInt(<any>width);
  };

  let width = 0;

  if (LOCK) {
    e.preventDefault();
    return;
  }

  try {
    LOCK = true;
    if (CONST.btn().checked == false) {
      width = 0;
      setWidth(width);
      CONST.messageText().before();
      CONST.btn().ariaChecked = <any>false;
      LOCK = false;
      return;
    }
    e.preventDefault();
    // steps:

    // 1. show during
    CONST.messageText().during();
    // 1. get config
    const config = await fetchPoWConfig();
    const max_recorded_nonce = config.max_recorded_nonce;
    // 2. prove work
    worker.postMessage(config);

    worker.onmessage = async (event: MessageEvent) => {
      const resp: ServiceWorkerMessage = event.data;

      if (resp.type === "work") {
        width = 80;
        setWidth(width);
        console.log(
          `Proof generated. Difficuly: ${config.difficulty_factor} Duration: ${resp.value.work.time}`
        );

        const proof: Work = {
          key: CONST.sitekey(),
          string: config.string,
          nonce: resp.value.work.nonce,
          result: resp.value.work.result,
          time: Math.trunc(resp.value.work.time),
          worker_type: resp.value.work.worker_type,
        };

        width = 90;
        setWidth(width);
        // 3. submit work
        const token = await sendWork(proof);
        // 4. send token
        sendToParent(token);
        // 5. mark checkbox checked
        CONST.btn().checked = true;
        CONST.btn().ariaChecked = <any>true;
        width = 100;
        setWidth(width);
        CONST.messageText().after();
        LOCK = false;
      }
      if (resp.type === "progress") {
        if (width < 80) {
          width = Number(resp.nonce / max_recorded_nonce) * 100;
          setWidth(width);
        }
        console.log(`received nonce ${resp.nonce}`);
      }
    };
  } catch (e) {
    CONST.messageText().error();
    console.error(e);
    LOCK = false;
  }
};

registerVerificationEventHandler();
