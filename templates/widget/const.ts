// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import LazyElement from "../utils/lazyElement";

/** mcaptcha checkbox ID **/
export const btnId = "widget__verification-checkbox";

/** get sitekey */
export const sitekey = (): string => {
  let sitekey;
  return (() => {
    if (sitekey === null || sitekey === undefined) {
      sitekey = new URL(window.location.href).searchParams.get("sitekey");
      if (sitekey === null || sitekey === undefined) {
        throw new Error("Define sitekey in query parameter");
      }
    }
    return sitekey;
  })();
};

/** mCaptcha API routes */
export const ROUTES = (() => {
  const getConfig = "/api/v1/pow/config";
  const verififyPoW = "/api/v1/pow/verify";

  return {
    /** get URL to fetch PoW configuration */
    getConfig,
    /** get URL to verify PoW*/
    verififyPoW,
  };
})();

/** get mCaptcha verifify checkbox button */
export const btn = (): HTMLInputElement => {
  let btn;
  return (() => {
    if (btn === null || btn === undefined) {
      btn = <HTMLInputElement>document.getElementById(btnId);
      if (btn === null || btn === undefined) {
        throw new Error("mCaptcha button not found)");
      }
    }
    return btn;
  })();
};

type messageTextReturn = {
  before: () => void;
  after: () => void;
  during: () => void;
  error: () => void;
};

export const BEFORE = "I'm not a robot";
export const DURING = "Processing...";
export const AFTER = "Verified!";
export const ERROR = "Something went wrong";

export const messageText = (): messageTextReturn => {
  const conatinerID = "widget__verification-text";

  const container = new LazyElement(conatinerID);

  /** runner fn to display HTMLElement **/
  const showMsg = (value: string) => {
    container.get().innerText = value;
    btn().ariaValueText = value;
  };

  return {
    /** display "before" message **/
    before: () => {
      showMsg(BEFORE);
    },

    /** display "after" message **/
    after: () => {
      showMsg(AFTER);
    },

    /** display "during" message **/
    during: () => {
      showMsg(DURING);
    },

    /** display "error" message **/
    error: () => {
      showMsg(ERROR);
    },
  };
};

export const inputId = "mcaptcha-response";
