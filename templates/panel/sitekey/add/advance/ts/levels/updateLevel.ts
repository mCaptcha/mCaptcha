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

import CONST from "../const";
import getLevelFields from "./getLevelFields";
import { LEVELS } from "./index";

import createError from "../../../../../../components/error";

/** on-change event handler to update level */
const updateLevel = (e: Event): void => {
  const target = <HTMLInputElement>e.target;

  const id = target.id;

  let level;
  if (id.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
    level = parseInt(id.slice(CONST.VISITOR_WITHOUT_LEVEL.length));
  }
  if (id.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
    level = parseInt(id.slice(CONST.DIFFICULTY_WITHOUT_LEVEL.length));
  }

  if (Number.isNaN(level)) {
    console.error("[updateLevel.ts] level # computed is not correct, got NaN");
  }

  try {
    const updatedLevel = getLevelFields(level);
    LEVELS.update(updatedLevel, level);
  } catch (e) {
    createError(e.message);
  }
};

/** registers on-change event handlers to update levels */
export const register = (id: number): void => {
  const visitorID = CONST.VISITOR_WITHOUT_LEVEL + id.toString();
  const difficultyID = CONST.DIFFICULTY_WITHOUT_LEVEL + id.toString();

  const visitorElement = <HTMLInputElement>document.getElementById(visitorID);
  const difficultyElement = <HTMLInputElement>(
    document.getElementById(difficultyID)
  );

  visitorElement.addEventListener("input", updateLevel, false);
  difficultyElement.addEventListener("input", updateLevel, false);
};
