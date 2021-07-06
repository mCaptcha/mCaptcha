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
  let beforeClass = 'widget__verification-text--before';
  let duringClass = 'widget__verification-text--during';
  let errorClass = 'widget__verification-text--error';
  let afterClass = 'widget__verification-text--after';

  let before: HTMLElement;
  let after: HTMLElement;
  let during: HTMLElement;
  let error: HTMLElement;

  /** runner fn to display HTMLElement **/
  const showMsg = (e: HTMLElement) => (e.style.display = 'block');
  /** runner fn to hide HTMLElement **/
  const hideMsg = (e: HTMLElement) => (e.style.display = 'none');

  /** lazy init and get before elementt **/
  const getBefore = () => {
    if (before === null || before === undefined) {
      before = <HTMLElement>document.querySelector(`.${beforeClass}`);
      if (before === null || before === undefined) {
        throw new Error(`before element not found)`);
      }
      return before;
    }
  };

  /** lazy init and get after elementt **/
  const getAfter = () => {
    if (after === null || after === undefined) {
      after = <HTMLSpanElement>document.querySelector(`.${afterClass}`);
      if (after === null || after === undefined) {
        throw new Error(`after element not found)`);
      }
    }

    return after;
  };

  /** lazy init and get error elementt **/
  const getError = () => {
    if (error === null || error === undefined) {
      error = <HTMLSpanElement>document.querySelector(`.${errorClass}`);
      if (error === null || error === undefined) {
        throw new Error(`before error not found)`);
      }
    }
    return error;
  };

  /** lazy init and get during elementt **/
  const getDuring = () => {
    if (during === null || during === undefined) {
      during = <HTMLSpanElement>document.querySelector(`.${duringClass}`);
      if (during === null || during === undefined) {
        throw new Error(`before during not found)`);
      }
    }

    return during;
  };
  return {
    /** display "before" message **/
    before: () => {
      showMsg(getBefore());
      hideMsg(getAfter());
      hideMsg(getDuring());
      hideMsg(getError());
    },

    /** display "after" message **/
    after: () => {
      hideMsg(getBefore());
      showMsg(getAfter());
      hideMsg(getDuring());
      hideMsg(getError());
    },

    /** display "during" message **/
    during: () => {
      hideMsg(getBefore());
      hideMsg(getAfter());
      showMsg(getDuring());
      hideMsg(getError());
    },

    /** display "error" message **/
    error: () => {
      hideMsg(getBefore());
      hideMsg(getAfter());
      hideMsg(getDuring());
      showMsg(getError());
    },
  };
};

export const inputId = 'mcaptcha-response';
