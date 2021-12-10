/*
 * Copyright (C) 221  Aravinth Manivannan <realaravinth@batsense.net>
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

import { trim } from "../../setupTests";
import updateLabels from "./updateLabel";
import CONST from "../../const";

import log from "../../../../../../../logger";
import { MODE } from "../../../../../../../logger";

/** get initial form to test remove button functionality */
export const labelLevel = (level: number): string => {
  return `
<form class="sitekey-form" action="/api/v1/mcaptcha/levels/add" method="post">
  <fieldset class="sitekey__level-container" id="level-group-2">
    <legend class="sitekey__level-title">
      Level 2
    </legend>
    <label class="sitekey-form__level-label" for="visitor${level}"
      >Visitor
      <input
        class="sitekey-form__level-input"
        type="number"
        name="visitor2"
        
        id="visitor2"
      >
    </label>

    <label class="sitekey-form__level-label" for="difficulty${level}">
      Difficulty
      <input
        type="number"
        name="difficulty2"
        class="sitekey-form__level-input"
        
        id="difficulty2"
      >
    </label>
    <label class="sitekey-form__level-label--hidden" for="remove-level${level}">
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

it("addLevelButton works", () => {
  // removing level  2
  const level = 2;
  const levelGroup = document.querySelector(
    `#${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${level}`
  );

  const newLevel = 20;

  updateLabels(levelGroup, newLevel);

  const labels = <NodeListOf<HTMLLabelElement>>(
    levelGroup.querySelectorAll(`.${CONST.LABEL_CLASS}`)
  );
  log.log(labels);
  labels.forEach((label) => {
    log.log(`${label.htmlFor}`);
    if (label.htmlFor.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
      expect(label.htmlFor).toBe(`${CONST.VISITOR_WITHOUT_LEVEL}${newLevel}`);
    } else if (label.htmlFor.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
      expect(label.htmlFor).toBe(
        `${CONST.DIFFICULTY_WITHOUT_LEVEL}${newLevel}`
      );
    } else if (
      label.htmlFor.includes(CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL)
    ) {
      expect(label.htmlFor).toBe(
        `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${newLevel}`
      );
    } else {
      throw new Error("Did you add an extra label to DOM?");
    }
  });

  expect(trim(document.body.innerHTML)).toBe(trim(labelLevel(newLevel)));
});
