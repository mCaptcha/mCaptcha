// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
"use strict";

import getFormUrl from "./getFormUrl";
import {getLoginFormHtml} from "../setUpTests";

const formClassName = "form__box";
const formURL = "/api/v1/signin";

const noFormErr = "Can't find form";

document.body.innerHTML = getLoginFormHtml();

const form = document.querySelector("form");
form.action = formURL;
form.className = formClassName;

it("getFromUrl workds", () => {
  const name = `.${formClassName}`;
  expect(getFormUrl(name)).toContain(formURL);

  const form = <HTMLFormElement>document.querySelector("form");
  expect(getFormUrl(form)).toContain(formURL);

  expect(getFormUrl()).toContain(formURL);

  try {
    document.body.innerHTML = formURL;
    getFormUrl();
  } catch (e) {
    expect(e.message).toContain(noFormErr);
  }
});
