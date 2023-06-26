// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import * as CONST from "../const";

export const sitekey = "imbatman";

export const checkbox = <HTMLInputElement>document.createElement("input");
checkbox.type = "checkbox";
checkbox.id = CONST.btnId;

const getMessages = (state: string) => {
  const msg = <HTMLElement>document.createElement("span");
  msg.id = `widget__verification-text--${state}`;
  return msg;
};

export const beforeMsg = getMessages("before");
export const afterMsg = getMessages("after");
export const duringMsg = getMessages("during");
export const errorMsg = getMessages("error");

/** get base HTML with empty mCaptcha container */
export const getBaseHtml = (): HTMLFormElement => {
  const form = <HTMLFormElement>document.createElement("form");
  form.appendChild(checkbox);
  form.appendChild(beforeMsg);
  form.appendChild(duringMsg);
  form.appendChild(afterMsg);
  form.appendChild(errorMsg);

  return form;
};
