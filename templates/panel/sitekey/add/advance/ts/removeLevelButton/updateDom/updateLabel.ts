// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import CONST from "../../const";
import log from "../../../../../../../logger";

/** update level lables to match new level */
const updateLabels = (levelGroup: Element, newLevel: number): void => {
  // rename labels
  const labels = <NodeListOf<HTMLLabelElement>>(
    levelGroup.querySelectorAll("label")
  );
  log.log(labels);
  labels.forEach((label) => {
    log.log(`${label.htmlFor}`);
    const currentFor = label.htmlFor;
    if (currentFor.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
      label.htmlFor = `${CONST.VISITOR_WITHOUT_LEVEL}${newLevel}`;
    } else if (currentFor.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
      label.htmlFor = `${CONST.DIFFICULTY_WITHOUT_LEVEL}${newLevel}`;
    } else if (
      currentFor.includes(CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL)
    ) {
      label.htmlFor = `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${newLevel}`;
    } else {
      if (currentFor != "add") {
        throw new Error(
          `Did you add an extra label to DOM? Found label with for: ${currentFor}`
        );
      }
    }
  });
};

export default updateLabels;
