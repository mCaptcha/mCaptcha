// Copyright (C) 221  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {trim} from "../../setupTests";
import updateRemoveButton from "./updateRemoveButton";
import CONST from "../../const";

import log from "../../../../../../../logger";
import {MODE} from "../../../../../../../logger";

/** get initial form to test remove button functionality */
export const labelLevel = (level: number): string => {
  return `
<form class="sitekey-form" action="/api/v1/mcaptcha/levels/add" method="post">
  <fieldset class="sitekey__level-container" id="level-group-">
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
        name="remove-level${level}"
        id="remove-level${level}"
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

const level = 2;
document.body.innerHTML = labelLevel(level);

log.setMode(MODE.none);

it("update remove button works", () => {
  // removing level  2

  const levelGroup = document.getElementById(
    `${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}`,
  );

  const newLevel = 20;
  updateRemoveButton(levelGroup, newLevel);

  const button = <HTMLInputElement>(
    levelGroup.querySelector(`.${CONST.REMOVE_LEVEL_BUTTON_CLASS}`)
  );
  expect(button.id).toBe(
    `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${newLevel}`,
  );
  expect(button.name).toBe(
    `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${newLevel}`,
  );

  expect(trim(document.body.innerHTML)).toBe(trim(labelLevel(newLevel)));
});
