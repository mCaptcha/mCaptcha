/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
