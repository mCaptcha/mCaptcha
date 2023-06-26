// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import getNumLevels from "../../levels/getNumLevels";
import { getAddForm, addLevel } from "../../setupTests";

document.body.innerHTML = getAddForm();

export const setupAddlevels = (): void => {
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
