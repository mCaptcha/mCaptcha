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

import getLevelFields from './getLevelFields';
import {getAddForm, addLevel} from '../setupTests';
import {Level} from './index';
//import CONST from '../const';

document.body.innerHTML = getAddForm();

const level1: Level = {
  difficulty_factor: 200,
  visitor_threshold: 500,
};

const level2: Level = {
  difficulty_factor: 400,
  visitor_threshold: 700,
};

it('get levels fields works', () => {
  addLevel(level1.visitor_threshold, level1.difficulty_factor);
  console.log(document.body.innerHTML);
  expect(getLevelFields(1)).toEqual(level1);

  addLevel(level2.visitor_threshold, level2.difficulty_factor);
  expect(getLevelFields(2)).toEqual(level2);
});
