// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import * as CONST from "../const";

export const sitekey = "imbatman";

export const checkbox = <HTMLInputElement>document.createElement("input");
checkbox.type = "checkbox";
checkbox.id = CONST.btnId;

const getMessages = () => {
  const msg = <HTMLElement>document.createElement("span");
  msg.id = "widget__verification-text";
  msg.innerText = "I'm not a robot";
  return msg;
};

export const Msg = getMessages();

/** get base HTML with empty mCaptcha container */
export const getBaseHtml = (): HTMLFormElement => {
  const form = <HTMLFormElement>document.createElement("form");
  form.appendChild(checkbox);
  form.appendChild(Msg);
  return form;
};
