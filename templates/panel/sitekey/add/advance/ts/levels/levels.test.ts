// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {LEVELS, Level} from "./index";
import {level1, level1visErr, level1diffErr, level2} from "../setupTests";

const visitorErr = "visitor count should be greater than previous levels";
const difficultyErr = "difficulty should be greater than previous levels";

const zeroVisError = "visitors must be greater than zero";
const zeroDiffError = "difficulty must be greater than zero";

const zeroVis: Level = {
  difficulty_factor: 10,
  visitor_threshold: 0,
};

const zeroDiff: Level = {
  difficulty_factor: 0,
  visitor_threshold: 10,
};

it("LEVELS works", () => {
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
    expect(e.message).toContain(zeroVisError);
  }
  // difficulty is 0
  try {
    LEVELS.add(zeroDiff);
  } catch (e) {
    expect(e.message).toContain(zeroDiffError);
  }
});
