// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
"use strict";

import isNumber from "./isNumber";

it("getFromUrl workds", () => {
  expect(isNumber("test")).toBe(false);
  expect(isNumber("1test213")).toBe(false);

  expect(isNumber("12")).toBe(true);
  expect(isNumber(2)).toBe(true);
});
