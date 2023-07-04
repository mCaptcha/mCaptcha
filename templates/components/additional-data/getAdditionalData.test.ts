// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import additionalData from "./index";

it("sudo form works", () => {
  try {
    additionalData();
  } catch (e) {
    expect(e.message).toBe(
      "Couldn't retrieve additional data element, is the component loaded?",
    );
  }

  const element = document.createElement("div");
  element.id = "additional-data";
  document.body.appendChild(element);
  expect(additionalData()).toBe(element);
});
