/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
