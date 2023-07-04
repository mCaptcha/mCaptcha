// Copyright © 2021 Aravinth Manivnanan <realaravinth@batsense.net>.
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import genJsonPayload from "../utils/genJsonPayload";
import * as CONST from "./const";
import { Work, Token } from "./types";

export const sendWork = async (payload: Work): Promise<Token> => {
  try {
    const res = await fetch(CONST.ROUTES.verififyPoW, genJsonPayload(payload));
    if (res.ok) {
      console.debug("work verified");
      const token: Token = await res.json();
      console.debug(`token ${token.token}`);
      return token;
    } else {
      const err = await res.json();
      console.error(`error: ${err.error}`);
      throw new Error(err);
    }
  } catch (err) {
    CONST.messageText().error();
    console.error(err);
    await new Promise((r) => setTimeout(r, 1000));
    window.location.reload();
    throw err;
  }
};

export default sendWork;
