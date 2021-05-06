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

import validateLevel from './validateLevel';
import {getAddForm, level1, fillAddLevel} from '../setupTests';

document.body.innerHTML = getAddForm();

it('validate levels fields works', () => {
  // null error
  expect(validateLevel(1)).toEqual(false);

  fillAddLevel(level1.visitor_threshold, level1.difficulty_factor);
  expect(validateLevel(1)).toEqual(true);

  // zero visitor error
  fillAddLevel(0, level1.difficulty_factor);
  expect(validateLevel(1)).toEqual(false);

  // zero difficulty error
  fillAddLevel(level1.visitor_threshold, 0);
  expect(validateLevel(1)).toEqual(false);
});
