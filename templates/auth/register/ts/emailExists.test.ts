// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import fetchMock from "jest-fetch-mock";

import emailExists from "./emailExists";

import {mockAlert, getRegistrationFormHtml} from "../../../setUpTests";

import setup from "../../../components/error/setUpTests";

fetchMock.enableMocks();
mockAlert();

beforeEach(() => {
  fetchMock.resetMocks();
});

it("finds exchange", async () => {
  fetchMock.mockResponseOnce(JSON.stringify({exists: true}));

  document.body.innerHTML = getRegistrationFormHtml();
  document.querySelector("body").appendChild(setup());

  const emailField = <HTMLInputElement>document.getElementById("email");
  emailField.setAttribute("value", "test@a.com");

  expect(await emailExists()).toBe(true);

  fetchMock.mockResponseOnce(JSON.stringify({exists: false}));
  expect(await emailExists()).toBe(false);
});
