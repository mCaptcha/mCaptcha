// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
"use strict";

import createError from "./index";
import * as e from "./index";

import setup from "./setUpTests";


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
