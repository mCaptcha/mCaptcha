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
import isNumber from '../../../../../utils/isNumber';

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

import validateDuration from './validateDuration';
import {getAddForm, fillDuration} from '../setupTests';

document.body.innerHTML = getAddForm();

const emptyErr = "can't be empty";
const NaNErr = 'duration can contain nubers only';
const zeroErr = 'duration must be greater than zero';

const duration = 30;

it('validateDuration workds', () => {
  try {
    const event = new Event('submit');
    validateDuration(event);
  } catch (e) {
    expect(e.message).toContain(emptyErr);
  }

  // fill string error
  try {
    fillDuration('testing');
    const event = new Event('submit');
    validateDuration(event);
  } catch (e) {
    expect(e.message).toContain(NaNErr);
  }

  // zero err
  try {
    fillDuration(0);
    const event = new Event('submit');
    validateDuration(event);
  } catch (e) {
    expect(e.message).toContain(zeroErr);
  }

  fillDuration(duration);
  const event = new Event('submit');
  expect(validateDuration(event)).toBe(duration);
});
