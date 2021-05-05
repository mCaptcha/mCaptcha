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

import getNumLevels from './getNumLevels';

/** Datatype represenging an mCaptcha level */
export type Level = {
  difficulty_factor: number;
  visitor_threshold: number;
};

/** Datatype representing a collection of mCaptcha levels */
class Levels {
  levels: Array<Level>;
  numOnScreen: number;
  numRecoreded: number;

  constructor() {
    this.levels = [];
    this.numRecoreded = 0;
    this.numOnScreen = getNumLevels();
  }

  add = (newLevel: Level) => {
    console.debug(`[levels/index.ts] levels lenght: ${this.levels.length}`);
    if (newLevel.difficulty_factor <= 0) {
      throw new Error('Difficulty must be greater than zero');
    }

    if (newLevel.visitor_threshold <= 0) {
      throw new Error('Visitors must be graeter than zero');
    }

    if (this.levels.length == 0) {
      this.levels.push(newLevel);
      return true;
    }

    let msg;
    let count = 1;

    const validate = (level: Level, newLevel: Level) => {
      if (level.visitor_threshold >= newLevel.visitor_threshold) {
        msg = `Level: ${newLevel} visitor count has to greater than previous levels. See ${count}`;
        return true;
      }

      if (level.difficulty_factor >= newLevel.difficulty_factor) {
        msg = `Level ${this.levels.length} difficulty has to greater than previous levels See ${count}`;
        return true;
      }
      count++;
      return false;
    };

    if (this.levels.find(level => validate(level, newLevel))) {
      alert(msg);
      throw new Error(msg);
    } else {
      this.levels.push(newLevel);
      this.numOnScreen += 1;
      this.numRecoreded += 1;
    }
  };

  get = () => this.levels;
}

/** Singleton that does manipulations on Levels object */
export const LEVELS = (function() {
  const levels = new Levels();

  return {
    /** get levels */
    getLevels: () => levels.get(),
    /**
     * get levels displayed on screen.
     * This includes the one with add level button
     * */
    getOnScreen: () => levels.numOnScreen,
    /**
     * get levels recorded using LEVELS
     * This excludes the one with add level button
     * */
    getRecored: () => levels.numRecoreded,

    /** add new level */
    add: (newLevel: Level) => levels.add(newLevel),

    /** update levels */
    update: (updateLevel: Level, id: number) => {
      const tmpLevel = new Levels();

      id -= 1;
      try {
        for (let i = 0; i < levels.levels.length; i++) {
          if (id == i) {
            tmpLevel.add(updateLevel);
          } else {
            tmpLevel.add(levels.levels[i]);
          }
        }
        levels.levels = tmpLevel.levels;
        console.log(`post update:`);
        LEVELS.print();
        return true;
      } catch (e) {
        console.log(e);
        return false;
      }
    },

    print: () =>
      levels.levels.forEach(level =>
        console.debug(
          `difficulty_factor: ${level.difficulty_factor} visitor ${level.visitor_threshold}`,
        ),
      ),

    /** remove level */
    remove: (id: number) => {
      console.debug(`[LEVELS] received order to remove ${id} element`);

      const tmpLevel = new Levels();

      id -= 1;
      try {
        for (let i = 0; i < levels.levels.length; i++) {
          if (id != i) {
            tmpLevel.add(levels.levels[i]);
          } else {
            console.debug(`[LEVELS] removing ${i} element`);
            const rmElement = levels.levels[i];
            console.debug(
              `[LEVELS] removing element: 
              difficulty_factor: ${rmElement.difficulty_factor} 
              visitor_threshold: ${rmElement.visitor_threshold}`,
            );
          }
        }
        levels.levels = tmpLevel.levels;
        console.debug('Post remove:');
        LEVELS.print();
        return true;
      } catch (e) {
        console.log(e);
        return false;
      }
    },
  };
})();
