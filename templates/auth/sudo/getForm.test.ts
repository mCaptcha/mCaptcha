// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import form from "./index";

it("sudo form works", () => {
  try {
    form.get();
  } catch (e) {
    expect(e.message).toBe("Element form is undefined");
  }

  const element = document.createElement("form");
  element.id = "form";
  document.body.appendChild(element);
  expect(form.get()).toBe(element);
});
