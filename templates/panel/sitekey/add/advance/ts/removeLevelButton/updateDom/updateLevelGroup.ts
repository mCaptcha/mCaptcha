// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import CONST from "../../const";

/** update level grup to match new level */
const updateLevelGroup = (levelGroup: Element, newLevel: number): string =>
  (levelGroup.id = `${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${newLevel}`);

export default updateLevelGroup;
