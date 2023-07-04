// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import getNumLevels from "./getNumLevels";
import {getAddForm, addLevel} from "../setupTests";

document.body.innerHTML = getAddForm();

it("get num levels works", () => {
  expect(getNumLevels()).toBe(1);
  addLevel(2, 4);
  expect(getNumLevels()).toBe(2);
  addLevel(4, 9);
  expect(getNumLevels()).toBe(3);
});
