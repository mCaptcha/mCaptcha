// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import { stepped_gen_pow } from "@mcaptcha/pow-wasm";
import * as p from "@mcaptcha/pow_sha256-polyfill";
import { WasmWork, PoWConfig, SubmitWork } from "./types";

/**
 * proove work
 * @param {PoWConfig} config - the proof-of-work configuration using which
 * work needs to be computed
 * */
const prove = async (
  config: PoWConfig,
  progress: (nonce: number) => void
): Promise<SubmitWork> => {
  const WASM = "wasm";
  const JS = "js";
  const STEPS = 5000;
  if (WasmSupported) {
    let proof: WasmWork = null;
    let res: SubmitWork = null;
    let time: number = null;

    const t0 = performance.now();
    const proofString = stepped_gen_pow(
      config.salt,
      config.string,
      config.difficulty_factor,
      STEPS,
      (nonce: bigint | number) => progress(Number(nonce))
    );
    const t1 = performance.now();
    time = t1 - t0;

    proof = JSON.parse(proofString);
    const worker_type = WASM;
    res = {
      result: proof.result,
      nonce: proof.nonce,
      worker_type,
      time,
    };
    return res;
  } else {
    console.log("WASM unsupported, expect delay during proof generation");

    let proof: WasmWork = null;
    let time: number = null;
    let res: SubmitWork = null;

    const t0 = performance.now();

    proof = await p.stepped_generate_work(
      config.salt,
      config.string,
      config.difficulty_factor,
      STEPS,
      progress
    );
    const t1 = performance.now();
    time = t1 - t0;

    const worker_type = JS;
    res = {
      result: proof.result,
      nonce: proof.nonce,
      worker_type,
      time,
    };

    return res;
  }
};

// credits: @jf-bastien on Stack Overflow
// https://stackoverflow.com/questions/47879864/how-can-i-check-if-a-browser-supports-webassembly
const WasmSupported = (() => {
  try {
    if (
      typeof WebAssembly === "object" &&
      typeof WebAssembly.instantiate === "function"
    ) {
      const module = new WebAssembly.Module(
        Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00)
      );
      if (module instanceof WebAssembly.Module)
        return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
    }
  } catch (e) {
    console.error(e);
  }
  return false;
})();

export default prove;
