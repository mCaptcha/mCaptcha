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
import getNumLevels from '../../levels/getNumLevels';
import CONST from '../../const';
import log from '../../../../../../logger';

import updateLabels from './updateLabel';
import updateInputs from './updateInputs';
import updateRemoveButton from './updateRemoveButton';
import updateLevelGroup from './updateLevelGroup';

/**
 * update level number on fieldset legends and their ids too
 * @param {number} id - level number that was ordered to remove.
 * All updates are made relative to id
 * */
const updateLevelNumbersOnDOM = (id: number) => {
  const numLevels = getNumLevels();
  if (id == numLevels) {
    throw new Error(
      "Can't remove the very fist element, it has to be first added to DOM",
    );
  }

  // since I'm doing id+1, I have to remove id after I'm done
  // with inclreasing level numbers
  for (let i = id + 1; i <= numLevels; i++) {
    const newLevel = i - 1;

    const levelGroup = document.querySelector(
      `#${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${i}`,
    );

    if (levelGroup === null) {
      const msg = `[removeLevelButton.ts]:
      error when trying to fetch level group field set ${i}. got null`;
      log.error(msg);
      throw new Error(msg);
    }

    // rename legend
    const legendText = document.createTextNode(`Level ${newLevel}`);
    levelGroup.getElementsByTagName(
      'legend',
    )[0].appendChild(legendText);


    // rename labels
    updateLabels(levelGroup, newLevel);

    // rename inputs
    updateInputs(levelGroup, newLevel);

    if (i != numLevels) {
      // update remove button
      updateRemoveButton(levelGroup, newLevel);
    }

    // update levelGroup's ID
    updateLevelGroup(levelGroup, newLevel);
    // TODO change remove button ID as well

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

export default updateLevelNumbersOnDOM;
