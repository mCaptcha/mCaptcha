// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import getLevelFields from "./getLevelFields";
import {
  getAddForm,
  level1,
  level2,
  fillAddLevel,
  addLevel,
} from "../setupTests";

document.body.innerHTML = getAddForm();

const visNumErr = "visitor can contain nubers only";
const diffNumErr = "difficulty can contain nubers only";

it("get levels fields works", () => {
  addLevel(level1.visitor_threshold, level1.difficulty_factor);
  expect(getLevelFields(1)).toEqual(level1);

  // NaN visitor
  try {
    fillAddLevel("test", level2.difficulty_factor);
    getLevelFields(2);
  } catch (e) {
    expect(e.message).toBe(visNumErr);
  }

  // Nan difficulty_factor
  try {
    fillAddLevel(level2.visitor_threshold, "fooasdads");
    getLevelFields(2);
  } catch (e) {
    expect(e.message).toBe(diffNumErr);
  }

  addLevel(level2.visitor_threshold, level2.difficulty_factor);
  expect(getLevelFields(2)).toEqual(level2);
});
