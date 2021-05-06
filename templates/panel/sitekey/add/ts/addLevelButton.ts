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
import validateLevel from './levels/validateLevel';
import getNumLevels from './levels/getNumLevels';
import  {LEVELS} from './levels/';
import * as UpdateLevel from './levels/updateLevel';
import {
  getRemoveButtonHTML,
  addRemoveLevelButtonEventListener,
} from './removeLevelButton';
import CONST from './const';


/**
 * Gets executed when 'Add' Button is clicked to add levels
 * Used to validate levels per m_captcha::defense::Defense's
 * specifications
 */
const addLevel = (e: Event) => {
  const eventTarget = <HTMLElement>e.target;
  const PARENT = <HTMLLabelElement>eventTarget.parentElement;
  const FIELDSET = <HTMLElement>PARENT.parentElement;
  const onScreenLevel = getNumLevels();

  const isValid = validateLevel(onScreenLevel);
  console.log(`[addLevelButton] isValid: ${isValid}`);
  if (!isValid) {
    return console.error('Aborting level addition');
  }

  eventTarget.remove();
  PARENT.innerHTML = getRemoveButtonHTML(onScreenLevel);
  PARENT.htmlFor = `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${onScreenLevel}`;
  //FIELDSET.innerHTML += getRemoveButtonHTML(numLevels);
  addRemoveLevelButtonEventListener(onScreenLevel);

  //PARENT.remove();

  const newLevelHTML = getHtml(onScreenLevel + 1);
  FIELDSET.insertAdjacentHTML('afterend', newLevelHTML);
  UpdateLevel.register(onScreenLevel);

  addLevelButtonAddEventListener();
};

/** adds onclick event listener */
const addLevelButtonAddEventListener = () => {
  const addLevelButton = <HTMLElement>(
    document.querySelector(`.${CONST.ADD_LEVEL_BUTTON}`)
  );
  addLevelButton.addEventListener('click', addLevel);
};

/**
 * Generate HTML to be added when 'Add Level' button is clicked
 * Check if './add-level.html` to see if this is up to date
 */
const getHtml = (level: number) => {
  console.debug(`[generating HTML getHtml]level: ${level}`);

  const HTML = `
<fieldset class="sitekey__level-container" id="level-group-${level}">
  <legend class="sitekey__level-title">
	  Level ${level}
  </legend>
  <label class="sitekey-form__level-label" for="visitor${level}"
    >Visitor
    <input
      class="sitekey-form__level-input"
      type="number"
      name=""
      value=""
      id="visitor${level}"
    />
  </label>

  <label class="sitekey-form__level-label" for="difficulty${level}">
    Difficulty
    <input
      type="number"
      name="difficulty"
      class="sitekey-form__level-input"
      value=""
      id="difficulty${level}"
    />
  </label>
  <label class="sitekey-form__level-label--hidden" for="add">
	  Add level
  <input
    class="sitekey-form__level-add-level-button"
    type="button"
    name="add"
    id="add"
    value="Add"
  />
  </label>
</fieldset>
`;
  return HTML;
};

export default addLevelButtonAddEventListener;
