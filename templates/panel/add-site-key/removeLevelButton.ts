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
import {LEVELS} from './levels/index';
import getNumLevels from './levels/getNumLevels';
import CONST from './const';

const REMOVE_LEVEL_BUTTON = 'sitekey-form__level-remove-level-button';

/**
 * Gets executed when 'Remove' Button is clicked to remove levels
 */
const removeLevel = (e: Event) => {
  const eventTarget = <HTMLElement>e.target;
  const PARENT = <HTMLElement>eventTarget.parentElement;
  const FIELDSET = <HTMLElement>PARENT.parentElement;

  const levelNum = parseInt(
    eventTarget.id.slice(CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL.length),
  );

  if (Number.isNaN(levelNum)) {
    const msg =
      '[removeLevelButton.ts] error in parsing level number from remove button ID';
    //console.error(msg);
    throw new Error(msg);
  }
  updateLevelNumbersOnDOM(levelNum);

  LEVELS.remove(levelNum);
  FIELDSET.remove();
};

/** update level number on fieldset legends and their ids too */
const updateLevelNumbersOnDOM = (id: number) => {
  const numLevels = getNumLevels();
  if (id + 1 == numLevels) {
    // this is the first elemet so have to remove fist element 
    // and downgrade  the add thingy

  }

  // since I'm doing id+1, I have to remove id after I'm done
  // with inclreasing level numbers
  for (let i = id+1; i <= numLevels; i++) {
    const newLevel = i-1;

    const levelGroup = document.querySelector(
      `#${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${i}`,
    );

    if (levelGroup === null) {
      const msg = `[removeLevelButton.ts]:
      error when trying to fetch level group field set ${i}. got null`;
      //console.error(msg);
      throw new Error(msg);
    }

    // rename legend
    levelGroup.getElementsByTagName(
      'legend',
    )[0].innerText = `Level ${newLevel}`;

    // rename labels
    const labels = <NodeListOf<HTMLLabelElement>>(
      levelGroup.querySelectorAll(`.${CONST.LABEL_CLASS}`)
    );
    //console.log(labels);
    labels.forEach(label => {
      //console.log(`${label.htmlFor}`);
      if (label.htmlFor.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
        label.htmlFor = `${CONST.VISITOR_WITHOUT_LEVEL}${newLevel}`;
      }

      if (label.htmlFor.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
        label.htmlFor = `${CONST.DIFFICULTY_WITHOUT_LEVEL}${newLevel}`;
      }
    });

    // rename inputs
    const inputs = <NodeListOf<HTMLInputElement>>(
      levelGroup.querySelectorAll(`.${CONST.LEVEL_INPUT_CLASS}`)
    );
    //console.log(inputs);
    inputs.forEach(input => {
      if (input.id.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
        //console.log(`${input.id}`);
        //console.log('changing visitor_threshold input');
        input.id = `${CONST.VISITOR_WITHOUT_LEVEL}${newLevel}`;
      }

      if (input.id.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
        //console.log('changing difficulty input');
        input.id = `${CONST.DIFFICULTY_WITHOUT_LEVEL}${newLevel}`;
      }
    });

    levelGroup.id = `${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${newLevel}`;

    /* TODO
     * change field set ID
     * change legend inner Text
     * change visitor lable for value
     * change visitor input id
     * change difficulty for value
     * change difficulty input id
     */
  }
};

/** adds onclick event listener */
export const addRemoveLevelButtonEventListener = (level: number) => {
  const removeButton = <HTMLElement>(
    document.querySelector(
      `#${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${level}`,
    )
  );
  removeButton.addEventListener('click', removeLevel);
};

/** adds onclick event listener to all remove buttons */
export const addRemoveLevelButtonEventListenerAll = () => {
  const removeButtons = document.querySelectorAll(`.${REMOVE_LEVEL_BUTTON}`);
  removeButtons.forEach(button =>
    button.addEventListener('click', removeLevel),
  );
};

/**
 * Generate Remove button HTML. On-click handler should be added
 * seprately
 */
export const getRemoveButtonHTML = (level: number) => {
  //console.debug(`[generating HTML getHtml]level: ${level}`);
  const HTML = `
  ${CONST.REMOVE_LEVEL_LABEL_TEXT}
  <input
    class="sitekey-form__level-remove-level-button"
    type="button"
    name="${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${level}"
    id="${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${level}"
    value="x"
  />
</fieldset>
`;
  return HTML;
};
