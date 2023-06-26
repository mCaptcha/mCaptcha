// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
