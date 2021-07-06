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

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pow_generation_works() {
    use mcaptcha_browser::*;
    use pow_sha256::*;

    const SALT: &str = "yrandomsaltisnotlongenoug";
    const PHRASE: &str = "ironmansucks";
    const DIFFICULTY: u32 = 1000;
    let serialised_work = gen_pow(SALT.into(), PHRASE.into(), DIFFICULTY);

    let work: Work = serde_json::from_str(&serialised_work).unwrap();

    let work = PoWBuilder::default()
        .result(work.result)
        .nonce(work.nonce)
        .build()
        .unwrap();

    let config = ConfigBuilder::default().salt(SALT.into()).build().unwrap();
    assert!(config.is_valid_proof(&work, &PHRASE.to_string()));
    assert!(config.is_sufficient_difficulty(&work, DIFFICULTY));
}
