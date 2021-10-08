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

import getNumLevels from "../levels/getNumLevels";
import { getAddForm, addLevel } from "../setupTests";
import CONST from "../const";

import log from "../../../../../logger";
import { MODE } from "../../../../../logger";

document.body.innerHTML = getAddForm();

const setUp = () => {
  expect(getNumLevels()).toBe(1);
  // add a level
  addLevel(2, 2);
  expect(getNumLevels()).toBe(2);

  // add second level
  addLevel(4, 4);
  expect(getNumLevels()).toBe(3);

  // add thrid level
  addLevel(5, 5);
  expect(getNumLevels()).toBe(4);
};

log.setMode(MODE.none);

it("removeLevelButton works", () => {
  setUp();

  for (let i = 1; i < 4; i++) {
    const l1 = <HTMLButtonElement>(
      document.getElementById(
        `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${1}`
      )
    );

    const expecting = 4 - i;

    l1.click();
    const currentLevels = getNumLevels();
    expect(currentLevels).toBe(expecting);
  }
});
