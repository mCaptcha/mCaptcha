/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
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

import log from "../../../../../logger";

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
