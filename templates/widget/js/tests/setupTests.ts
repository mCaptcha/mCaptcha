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
import * as CONST from '../const';

export const sitekey = 'imbatman';

export const checkbox = <HTMLInputElement>document.createElement('input');
checkbox.type = 'checkbox';
checkbox.id = CONST.btnId;

const getMessages = (state: string) => {
  const msg = <HTMLElement>document.createElement('span');
  msg.className = `widget__verification-text--${state}`;
  return msg;
};

export const beforeMsg = getMessages('before');
export const afterMsg = getMessages('after');
export const duringMsg = getMessages('during');
export const errorMsg = getMessages('error');

/** get base HTML with empty mCaptcha container */
export const getBaseHtml = () => {
  const form = <HTMLFormElement>document.createElement('form');
  form.appendChild(checkbox);
  form.appendChild(beforeMsg);
  form.appendChild(duringMsg);
  form.appendChild(afterMsg);
  form.appendChild(errorMsg);

  return form;
};
