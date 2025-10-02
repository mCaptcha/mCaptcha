// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
"use strict";

import isBlankString from "./isBlankString";
import {mockAlert} from "../setUpTests";


import setup from "../components/error/setUpTests";


mockAlert();

it("getFromUrl workds", () => {
  document.querySelector("body").appendChild(setup());
  expect(isBlankString("test", "username")).toBe(false);
  try {
    isBlankString("  ", "username");
  } catch (e) {
    expect(e.message).toContain("can't be empty");
  }
});
