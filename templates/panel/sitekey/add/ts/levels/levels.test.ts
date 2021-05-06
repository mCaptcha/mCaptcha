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

import {LEVELS, Level} from './index';
import {level1, level1visErr, level1diffErr, level2} from '../setupTests';

const visitorErr = 'visitor count has to greater than previous levels';
const difficultyErr = 'difficulty has to greater than previous levels';

const zeroVisError = 'Visitors must be greater than zero';
const zeroDiffError = 'Difficulty must be greater than zero';

const zeroVis: Level = {
  difficulty_factor: 10,
  visitor_threshold: 0,
};

const zeroDiff: Level = {
  difficulty_factor: 0,
  visitor_threshold: 10,
};

it('LEVELS works', () => {
  // add level
  LEVELS.add(level1);
  expect(LEVELS.getLevels()).toEqual([level1]);

  // add visitor count < prev level
  try {
    LEVELS.add(level1visErr);
  } catch (e) {
    expect(e.message).toContain(visitorErr);
  }

  // add difficulty < prev level
  try {
    LEVELS.add(level1diffErr);
  } catch (e) {
    expect(e.message).toContain(difficultyErr);
  }

  // add second level
  LEVELS.add(level2);
  expect(LEVELS.getLevels()).toEqual([level1, level2]);

  // update level
  const newLevel2 = level2;
  newLevel2.difficulty_factor = 8000;
  LEVELS.update(newLevel2, 2);
  expect(LEVELS.getLevels()).toEqual([level1, newLevel2]);

  // update second level
  LEVELS.remove(1);
  expect(LEVELS.getLevels()).toEqual([newLevel2]);

  // visitor is 0
  try {
    LEVELS.add(zeroVis);
  } catch (e) {
    expect(e.message).toEqual(zeroVisError);
  }
  // difficulty is 0
  try {
    LEVELS.add(zeroDiff);
  } catch (e) {
    expect(e.message).toEqual(zeroDiffError);
  }
});
