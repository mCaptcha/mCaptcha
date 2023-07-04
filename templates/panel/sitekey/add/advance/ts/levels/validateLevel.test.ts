// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import validateLevel from "./validateLevel";
import {getAddForm, level1, fillAddLevel} from "../setupTests";
import setup from "../../../../../../components/error/setUpTests";

document.body.innerHTML = getAddForm();

document.body.appendChild(setup());

it("validate levels fields works", () => {
  // null error
  expect(validateLevel(1)).toEqual(false);

  fillAddLevel(level1.visitor_threshold, level1.difficulty_factor);
  expect(validateLevel(1)).toEqual(true);

  // zero visitor error
  fillAddLevel(0, level1.difficulty_factor);
  expect(validateLevel(1)).toEqual(false);

  // zero difficulty error
  fillAddLevel(level1.visitor_threshold, 0);
  expect(validateLevel(1)).toEqual(false);
});
