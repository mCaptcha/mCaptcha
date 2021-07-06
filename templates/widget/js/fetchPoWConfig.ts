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

import genJsonPayload from './utils/genJsonPayload';
import * as CONST from './const';

type GetConfigPayload = {
  key: string;
};

export type PoWConfig = {
  string: string;
  difficulty_factor: number;
  salt: string;
};

/**
 * fetch proof-of-work configuration
 * @returns {PoWConfig} pow config
 * */
export const fetchPoWConfig = async () => {
  try {
    const payload: GetConfigPayload = {
      key: CONST.sitekey(),
    };

    const res = await fetch(CONST.ROUTES.getConfig, genJsonPayload(payload));
    if (res.ok) {
      const config: PoWConfig = await res.json();
      return config;
    } else {
      const err = await res.json();
      throw new Error(err);
    }
  } catch (err) {
    throw err;
  }
};

export default fetchPoWConfig;
