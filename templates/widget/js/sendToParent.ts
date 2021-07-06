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
import {Token} from './sendWork';

/**
 * send pow validation token as message to parant of the iframe
 * @param {Token} token: token received from mCaptcha service
 * upon successful PoW validation
 * */
export const sendToParent = (token: Token) => {
  window.parent.postMessage(token, '*');
  // TODO set origin. Make parent send origin as query parameter
  // or as a message to iframe
};

export default sendToParent;
