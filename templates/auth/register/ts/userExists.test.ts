// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import fetchMock from "jest-fetch-mock";

import userExists from "./userExists";

import {mockAlert, getLoginFormHtml} from "../../../setUpTests";

import setup from "../../../components/error/setUpTests";

fetchMock.enableMocks();
mockAlert();

beforeEach(() => {
  fetchMock.resetMocks();
});

it("finds exchange", async () => {
  fetchMock.mockResponseOnce(JSON.stringify({exists: true}));

  document.body.innerHTML = getLoginFormHtml();
  document.querySelector("body").appendChild(setup());
  const usernameField = <HTMLInputElement>document.querySelector("#username");
  usernameField.value = "test";
  expect(await userExists()).toBe(true);

  usernameField.value = "test";
  fetchMock.mockResponseOnce(JSON.stringify({exists: true}));
  expect(await userExists(usernameField)).toBe(true);

  fetchMock.mockResponseOnce(JSON.stringify({exists: false}));
  expect(await userExists()).toBe(false);
});
