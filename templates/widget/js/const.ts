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
import LazyElement from '../../utils/lazyElement';

/** mcaptcha checkbox ID **/
export const btnId = 'widget__verification-checkbox';

/** get sitekey */
export const sitekey = () => {
  let sitekey;
  return (() => {
    if (sitekey === null || sitekey === undefined) {
      sitekey = new URL(window.location.href).searchParams.get('sitekey');
      if (sitekey === null || sitekey === undefined) {
        throw new Error(`Define sitekey in query parameter`);
      }
    }
    return sitekey;
  })();
};

/** mCaptcha API routes */
export const ROUTES = (() => {
  const getConfig = '/api/v1/pow/config';
  const verififyPoW = '/api/v1/pow/verify';

  return {
    /** get URL to fetch PoW configuration */
    getConfig,
    /** get URL to verify PoW*/
    verififyPoW,
  };
})();

/** get mCaptcha verifify checkbox button */
export const btn = () => {
  let btn;
  return (() => {
    if (btn === null || btn === undefined) {
      btn = <HTMLInputElement>document.getElementById(btnId);
      if (btn === null || btn === undefined) {
        throw new Error(`mCaptcha button not found)`);
      }
    }
    return btn;
  })();
};

export const messageText = () => {
  const beforeID = 'widget__verification-text--before';
  const duringID = 'widget__verification-text--during';
  const errorID = 'widget__verification-text--error';
  const afterID = 'widget__verification-text--after';

  const before = new LazyElement(beforeID);
  const after = new LazyElement(afterID);
  const during = new LazyElement(duringID);
  const error = new LazyElement(errorID);
  //  let before: HTMLElement;
  //  let after: HTMLElement;
  //  let during: HTMLElement;
  //  let error: HTMLElement;

  /** runner fn to display HTMLElement **/
  const showMsg = (e: HTMLElement) => (e.style.display = 'block');
  /** runner fn to hide HTMLElement **/
  const hideMsg = (e: HTMLElement) => (e.style.display = 'none');

  return {
    /** display "before" message **/
    before: () => {
      showMsg(before.get());
      hideMsg(after.get());
      hideMsg(during.get());
      hideMsg(error.get());
    },

    /** display "after" message **/
    after: () => {
      hideMsg(before.get());
      showMsg(after.get());
      hideMsg(during.get());
      hideMsg(error.get());
    },

    /** display "during" message **/
    during: () => {
      hideMsg(before.get());
      hideMsg(after.get());
      showMsg(during.get());
      hideMsg(error.get());
    },

    /** display "error" message **/
    error: () => {
      hideMsg(before.get());
      hideMsg(after.get());
      hideMsg(during.get());
      showMsg(error.get());
    },
  };
};

export const inputId = 'mcaptcha-response';
