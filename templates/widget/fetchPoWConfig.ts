// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import genJsonPayload from "../utils/genJsonPayload";
import * as CONST from "./const";
import { PoWConfig } from "./types";

type GetConfigPayload = {
  key: string;
};

/**
 * fetch proof-of-work configuration
 * @returns {PoWConfig} pow config
 * */
export const fetchPoWConfig = async (): Promise<PoWConfig> => {
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
};

export default fetchPoWConfig;
