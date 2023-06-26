// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import getNumLevels from "../levels/getNumLevels";
import { getAddForm, addLevel } from "../setupTests";
import CONST from "../const";

import log from "../../../../../../logger";
import { MODE } from "../../../../../../logger";

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
