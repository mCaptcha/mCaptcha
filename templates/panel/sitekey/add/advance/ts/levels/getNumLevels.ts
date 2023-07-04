// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import CONST from "../const";

import log from "../../../../../../logger";

/** returns number of level input fields currently in DOM */
const getNumLevels = (): number => {
  let numLevels = 0;
  document
    .querySelectorAll(`.${CONST.LEVEL_CONTAINER_CLASS}`)
    .forEach(() => numLevels++);
  log.debug(`[getNumLevels]: numLevels: ${numLevels}`);
  return numLevels;
};

export default getNumLevels;
