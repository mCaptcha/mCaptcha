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
//! mCaptcha is a proof of work based Denaial-of-Service attack protection system.
//! This is is a WASM library that you can embed in your frontend code to protect your
//! service.
//!
//! A commercial managed solution is in the works but I'd much rather prefer
//! folks host their own instances as it will make the more decentralized and free.
//!
//! ## Workflow:
//! mCaptcha workflow in the frontend is simple.
//! 1. Call service to get a proof of work(PoW) configuration
//! 2. Call into mCaptcha to get PoW
//! 3. Send PoW to mCaptcha service
//! 4. If proof is valid, the service will return a token to the client
//! 5. Submit token to your backend along with your app data(if any)
//! 6. In backend, validate client's token with mCaptcha service
//!
//! ## Example:
//!
//! generate proof-of-work
//! ```rust
//! fn main() {
//!    use mcaptcha_browser::*;
//!    use pow_sha256::*;
//!
//!
//!    // salt using which PoW should be computed
//!    const SALT: &str = "yrandomsaltisnotlongenoug";
//!    // one-time phrase over which PoW should be computed
//!    const PHRASE: &str = "ironmansucks";
//!    // and the difficulty factor
//!    const DIFFICULTY: u32 = 1000;
//!
//!    // currently gen_pow() returns a JSON formated string to better communicate
//!    // with JavaScript. See [PoW<T>][pow_sha256::PoW] for schema
//!    let serialised_work = gen_pow(SALT.into(), PHRASE.into(), DIFFICULTY);
//!
//!
//!    let work: Work = serde_json::from_str(&serialised_work).unwrap();
//!
//!    let work = PoWBuilder::default()
//!        .result(work.result)
//!        .nonce(work.nonce)
//!        .build()
//!        .unwrap();
//!
//!    let config = ConfigBuilder::default().salt(SALT.into()).build().unwrap();
//!    assert!(config.is_valid_proof(&work, &PHRASE.to_string()));
//!    assert!(config.is_sufficient_difficulty(&work, DIFFICULTY));
//! }
//! ```

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use pow_sha256::{ConfigBuilder, PoW};

#[derive(Deserialize, Serialize)]
pub struct Work {
    pub result: String,
    pub nonce: u64,
}

impl From<PoW<String>> for Work {
    fn from(p: PoW<String>) -> Self {
        Work {
            result: p.result,
            nonce: p.nonce,
        }
    }
}

/// generate proof-of-work
/// ```rust
/// fn main() {
///    use mcaptcha_browser::*;
///    use pow_sha256::*;
///
///
///    // salt using which PoW should be computed
///    const SALT: &str = "yrandomsaltisnotlongenoug";
///    // one-time phrase over which PoW should be computed
///    const PHRASE: &str = "ironmansucks";
///    // and the difficulty factor
///    const DIFFICULTY: u32 = 1000;
///
///    // currently gen_pow() returns a JSON formated string to better communicate
///    // with JavaScript. See [PoW<T>][pow_sha256::PoW] for schema
///    let serialised_work = gen_pow(SALT.into(), PHRASE.into(), DIFFICULTY);
///
///
///    let work: Work = serde_json::from_str(&serialised_work).unwrap();
///    
///    let work = PoWBuilder::default()
///        .result(work.result)
///        .nonce(work.nonce)
///        .build()
///        .unwrap();
///    
///    let config = ConfigBuilder::default().salt(SALT.into()).build().unwrap();
///    assert!(config.is_valid_proof(&work, &PHRASE.to_string()));
///    assert!(config.is_sufficient_difficulty(&work, DIFFICULTY));
/// }
/// ```
#[wasm_bindgen]
pub fn gen_pow(salt: String, phrase: String, difficulty_factor: u32) -> String {
    let config = ConfigBuilder::default().salt(salt).build().unwrap();

    let work = config.prove_work(&phrase, difficulty_factor).unwrap();
    let work: Work = work.into();

    let payload = serde_json::to_string(&work).unwrap();
    payload
}

#[cfg(test)]
mod tests {
    use super::*;
    use pow_sha256::PoWBuilder;

    const SALT: &str = "yrandomsaltisnotlongenoug";
    const PHRASE: &str = "ironmansucks";
    const DIFFICULTY: u32 = 1000;
    #[test]
    fn it_works() {
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
}
