// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import CONST from "../../const";
import log from "../../../../../../../logger";

/** update input IDs with new level */
const updateInput = (levelGroup: Element, newLevel: number): void => {
  const inputs = <NodeListOf<HTMLInputElement>>(
    levelGroup.querySelectorAll(`.${CONST.LEVEL_INPUT_CLASS}`)
  );
  log.log(inputs);
  inputs.forEach(input => {
    if (input.id.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
      log.log(`${input.id}`);
      log.log("changing visitor_threshold input");
      const id = `${CONST.VISITOR_WITHOUT_LEVEL}${newLevel}`;
      input.id = id;
      input.name = id;
    } else if (input.id.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
      log.log("changing difficulty input");
      const id = `${CONST.DIFFICULTY_WITHOUT_LEVEL}${newLevel}`;
      input.id = id;
      input.name = id;
    } else {
      if (input.id != "add") {
        throw new Error(`Did you add an extra input to DOM? ${input.id} ${input.className} ${input.name}`);
      }
    }
  });
};

export default updateInput;
