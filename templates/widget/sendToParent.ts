// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import {Token} from "./types";

/**
 * send pow validation token as message to parant of the iframe
 * @param {Token} token: token received from mCaptcha service
 * upon successful PoW validation
 * */
export const sendToParent = (token: Token): void => {
  window.parent.postMessage(token, "*");
  // TODO set origin. Make parent send origin as query parameter
  // or as a message to iframe
};

export default sendToParent;
