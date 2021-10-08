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

import createError from "./index";
import * as e from "./index";

import setup from "./setUpTests";

"use strict";

jest.useFakeTimers();

it("checks if error boxes work", () => {
  document.body.append(setup());

  const getMsg = (num: number) => `message ${num}`;
  createError(getMsg(1));
  let msg = document.querySelector(`.${e.ERR_MSG_CONTAINER}`);
  expect(msg.innerHTML).toContain(getMsg(1));

  const btn = <HTMLButtonElement>msg.getElementsByClassName(e.ERR_CLOSE)[0];
  btn.click();
  msg = document.querySelector(`.${e.ERR_MSG_CONTAINER}`);
  expect(msg).toEqual(null);

  const errElement = document.createElement("p");
  errElement.appendChild(document.createTextNode(getMsg(2)));
  createError(errElement);
  msg = document.querySelector(`.${e.ERR_MSG_CONTAINER}`).querySelector("p");
  expect(msg).toEqual(errElement);

  const timeOutElement = document.createElement("p");
  timeOutElement.appendChild(document.createTextNode(getMsg(2)));
  createError(timeOutElement, 200);
  msg = document.querySelector(`.${e.ERR_MSG_CONTAINER}`).querySelector("p");
  expect(msg).toEqual(timeOutElement);
  jest.runOnlyPendingTimers();
  msg = document.querySelector(`.${e.ERR_MSG_CONTAINER}`);
  expect(msg).toEqual(null);
});
