// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Level } from "./index";
import CONST from "../const";

import log from "../../../../../../logger";

/** Fetches level from DOM using the ID passesd and validates */
const getLevelFields = (id: number): Level => {
  log.debug(`[getLevelFields]: id: ${id}`);
  const visitorID = CONST.VISITOR_WITHOUT_LEVEL + id.toString();
  const difficultyID = CONST.DIFFICULTY_WITHOUT_LEVEL + id.toString();

  const visitorElement = <HTMLInputElement>document.getElementById(visitorID);
  const difficultyElement = <HTMLInputElement>(
    document.getElementById(difficultyID)
  );

  const visitor_threshold = parseInt(visitorElement.value);
  const difficulty_factor = parseInt(difficultyElement.value);

  if (Number.isNaN(visitor_threshold)) {
    throw new Error("visitor can contain nubers only");
  }

  if (Number.isNaN(difficulty_factor)) {
    throw new Error("difficulty can contain nubers only");
  }

  const level: Level = {
    difficulty_factor,
    visitor_threshold,
  };

  log.debug(
    `[getLevelFields.ts] visitor: ${visitor_threshold} difficulty: ${difficulty_factor}`
  );

  return level;
};

export default getLevelFields;
