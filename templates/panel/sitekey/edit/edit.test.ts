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

import getNumLevels from "../add/advance/ts/levels/getNumLevels";
import {addLevel} from "../add/advance/ts/setupTests";
import setup from "../../../components/error/setUpTests";
import * as SETUP from "./setupTest";

document.body.innerHTML = SETUP.EDIT_FORM;
document.body.appendChild(setup());

jest.useFakeTimers();

it("edit sitekey works", () => {
  expect(getNumLevels()).toBe(2);
  // add a level
  addLevel(5, 6);
  expect(getNumLevels()).toBe(3);

  // add second level
  addLevel(8, 9);
  expect(getNumLevels()).toBe(4);

  jest.runAllTimers();

  //  expect(trim(a)).toBe(trim(finalHtml()));

  // try to add negative parameters
  addLevel(-4, -9);
  expect(getNumLevels()).toBe(4);

  // try to add duplicate level
  addLevel(6, 7);
  expect(getNumLevels()).toBe(4);

});
