// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import validateDescription from "./validateDescription";
import {getAddForm, fillDescription} from "../setupTests";
import {mockAlert} from "../../../../../../setUpTests";

import setup from "../../../../../../components/error/setUpTests";

mockAlert();

document.body.innerHTML = getAddForm();

const emptyErr = "can't be empty";

it("validateDescription workds", () => {
  document.querySelector("body").appendChild(setup());
  try {
    const event = new Event("submit");
    validateDescription(event);
  } catch (e) {
    expect(e.message).toContain(emptyErr);
  }

  // fill and validate
  fillDescription("testing");
  const event = new Event("submit");
  validateDescription(event);
});
