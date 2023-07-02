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

export type Work = {
  result: string;
  nonce: number;
  string: string;
  key: string;
  time: number;
  worker_type: string;
};

export type SubmitWork = {
  time: number;
  worker_type: string;
  result: string;
  nonce: number;
};

export type WasmWork = {
  result: string;
  nonce: number;
};

export type ServiceWorkerWork = {
  work: SubmitWork;
};

export type PoWConfig = {
  string: string;
  difficulty_factor: number;
  salt: string;
};

export type Token = {
  token: string;
};
