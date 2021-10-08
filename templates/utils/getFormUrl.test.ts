/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
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

import getFormUrl from "./getFormUrl";
import {getLoginFormHtml} from "../setUpTests";

"use strict";

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
