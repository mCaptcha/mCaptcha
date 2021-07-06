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

import {getBaseHtml, sitekey, checkbox} from './setupTests';
import * as TESTElements from './setupTests';

it('const works', () => {
  const body = document.querySelector('body');
  const container = getBaseHtml();
  body.appendChild(container);
  expect(CONST.sitekey()).toBe(sitekey);
  expect(CONST.btn()).toBe(checkbox);

  // display after
  CONST.messageText().after();
  expect(TESTElements.afterMsg.style.display).toBe('block');
  expect(TESTElements.beforeMsg.style.display).toBe('none');
  expect(TESTElements.duringMsg.style.display).toBe('none');
  expect(TESTElements.errorMsg.style.display).toBe('none');

  // display before
  CONST.messageText().before();
  expect(TESTElements.afterMsg.style.display).toBe('none');
  expect(TESTElements.beforeMsg.style.display).toBe('block');
  expect(TESTElements.duringMsg.style.display).toBe('none');
  expect(TESTElements.errorMsg.style.display).toBe('none');

  // display during
  CONST.messageText().during();
  expect(TESTElements.afterMsg.style.display).toBe('none');
  expect(TESTElements.beforeMsg.style.display).toBe('none');
  expect(TESTElements.duringMsg.style.display).toBe('block');
  expect(TESTElements.errorMsg.style.display).toBe('none');

  // display error
  CONST.messageText().error();
  expect(TESTElements.afterMsg.style.display).toBe('none');
  expect(TESTElements.beforeMsg.style.display).toBe('none');
  expect(TESTElements.duringMsg.style.display).toBe('none');
  expect(TESTElements.errorMsg.style.display).toBe('block');
});
