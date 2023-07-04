// Copyright (C) 221  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { trim} from "../../setupTests";
import updateLevelGroup from "./updateLevelGroup";
import CONST from "../../const";

import log from "../../../../../../../logger";
import {MODE} from "../../../../../../../logger";


/** get initial form to test remove button functionality */
export const labelLevel = (level: number): string => {
  return `
<form class="sitekey-form" action="/api/v1/mcaptcha/levels/add" method="post">
  <fieldset class="sitekey__level-container" id="level-group-${level}">
    <legend class="sitekey__level-title">
      Level 2
    </legend>
    <label class="sitekey-form__level-label" for="visitor"
      >Visitor
      <input
        class="sitekey-form__level-input"
        type="number"
        name="visitor2"
        
        id="visitor2"
      >
    </label>

    <label class="sitekey-form__level-label" for="difficulty">
      Difficulty
      <input
        type="number"
        name="difficulty2"
        class="sitekey-form__level-input"
        
        id="difficulty2"
      >
    </label>
    <label class="sitekey-form__level-label--hidden" for="remove-level">
      Remove Level
      <input
        class="sitekey-form__level-remove-level-button"
        type="button"
        name="remove-level2"
        id="remove-level2"
        value="x"
      >
    </label>

<label class="sitekey-form__level-label--hidden" for="add">
	  Add level
  <input
    class="sitekey-form__level-add-level-button"
    type="button"
    name="add"
    id="add"
    value="Add"
  >
  </label>

  </fieldset>
</form>
`;
};

document.body.innerHTML = labelLevel(2);

log.setMode(MODE.none);

it("update levelGroup works", () => {
  // removing level  2
  const level = 2;
  const levelGroup = document.querySelector(
    `#${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${level}`,
  );

  const newLevel = 20;

  updateLevelGroup(levelGroup, newLevel);
  expect(levelGroup.id).toBe(
    `${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${newLevel}`,
  );

  expect(trim(document.body.innerHTML)).toBe(trim(labelLevel(newLevel)));
});
