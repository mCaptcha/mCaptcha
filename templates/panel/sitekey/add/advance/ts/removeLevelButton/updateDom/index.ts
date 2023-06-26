// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import getNumLevels from "../../levels/getNumLevels";
import CONST from "../../const";
import log from "../../../../../../../logger";

import updateLabels from "./updateLabel";
import updateInputs from "./updateInputs";
import updateRemoveButton from "./updateRemoveButton";
import updateLevelGroup from "./updateLevelGroup";

/**
 * update level number on fieldset legends and their ids too
 * @param {number} id - level number that was ordered to remove.
 * All updates are made relative to id
 * */
const updateLevelNumbersOnDOM = (id: number): void => {
  const numLevels = getNumLevels();
  if (id == numLevels) {
    throw new Error(
      "Can't remove the very fist element, it has to be first added to DOM"
    );
  }

  // since I'm doing id+1, I have to remove id after I'm done
  // with inclreasing level numbers
  for (let i = id + 1; i <= numLevels; i++) {
    const newLevel = i - 1;

    const levelGroup = document.querySelector(
      `#${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${i}`
    );

    if (levelGroup === null) {
      const msg = `[removeLevelButton.ts]:
      error when trying to fetch level group field set ${i}. got null`;
      log.error(msg);
      throw new Error(msg);
    }

    // rename legend
    const legend = levelGroup.getElementsByTagName("legend")[0];
    const legendText = document.createTextNode(`Level ${newLevel}`);
    const newLegend = document.createElement("legend");
    newLegend.className = legend.className;
    newLegend.appendChild(legendText);
    legend.replaceWith(newLegend);

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
