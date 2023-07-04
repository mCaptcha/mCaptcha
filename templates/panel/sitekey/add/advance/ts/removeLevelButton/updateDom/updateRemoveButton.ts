// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import CONST from "../../const";

/** update remove level button's ID */
const updateRemoveButton = (levelGroup: Element, newLevel: number): void => {
  // rename button
  const button = <HTMLInputElement>(
    levelGroup.querySelector(`.${CONST.REMOVE_LEVEL_BUTTON_CLASS}`)
  );
  const id = `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${newLevel}`;
  button.id = id;
  button.name = id;
};

export default updateRemoveButton;
