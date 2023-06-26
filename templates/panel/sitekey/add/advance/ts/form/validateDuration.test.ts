// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//const validateDuration = (e: Event) => {
//  const duartionElement = <HTMLInputElement>document.getElementById('duration');
//  const duration = parseInt(duartionElement.value);
//  if (!isNumber(duration) || Number.isNaN(duration)) {
//    throw new Error('duration can contain nubers only');
//  }
//
//  if (duration <= 0) {
//    throw new Error('duration must be greater than zero');
//  }
//  return duration;
//};
//
//export default validateDuration;

import validateDuration from "./validateDuration";
import {getAddForm, fillDuration} from "../setupTests";

document.body.innerHTML = getAddForm();

const emptyErr = "can't be empty";
const NaNErr = "duration can contain nubers only";
const zeroErr = "duration must be greater than zero";

const duration = 30;

it("validateDuration workds", () => {
  try {
    validateDuration();
  } catch (e) {
    expect(e.message).toContain(emptyErr);
  }

  // fill string error
  try {
    fillDuration("testing");
    validateDuration();
  } catch (e) {
    expect(e.message).toContain(NaNErr);
  }

  // zero err
  try {
    fillDuration(0);
    validateDuration();
  } catch (e) {
    expect(e.message).toContain(zeroErr);
  }

  fillDuration(duration);
  expect(validateDuration()).toBe(duration);
});
