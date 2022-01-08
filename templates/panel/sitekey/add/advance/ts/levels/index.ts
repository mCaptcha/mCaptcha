/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

import log from "../../../../../../logger";

/** Datatype represenging an mCaptcha level */
export type Level = {
  difficulty_factor: number;
  visitor_threshold: number;
};

/** Datatype representing a collection of mCaptcha levels */
class Levels {
  levels: Array<Level>;

  constructor() {
    this.levels = [];
  }

  add = (newLevel: Level) => {
    log.debug(`[levels/index.ts] levels lenght: ${this.levels.length}`);
    if (newLevel.difficulty_factor <= 0) {
      throw new Error(
        `Level ${this.levels.length}'s difficulty must be greater than zero`,
      );
    }

    if (newLevel.visitor_threshold <= 0) {
      throw new Error(
        `Level ${this.levels.length}'s visitors must be greater than zero`,
      );
    }

    if (this.levels.length == 0) {
      this.levels.push(newLevel);
      return true;
    }

    let count = 1;

    this.levels.forEach(level => {
      if (level.visitor_threshold >= newLevel.visitor_threshold) {
        const msg = `Level ${this.levels.length}'s visitor count should be greater than previous levels(Level ${count} is greater)`;
        throw new Error(msg);
      } else if (level.difficulty_factor >= newLevel.difficulty_factor) {
        const msg = `Level ${this.levels.length} difficulty should be greater than previous levels(Level ${count} is greater)`;
        throw new Error(msg);
      } else {
        count++;
      }
    });

    this.levels.push(newLevel);
  };

  get = () => this.levels;
}

/** Singleton that does manipulations on Levels object */
export const LEVELS = (function() {
  const levels = new Levels();

  return {
    /** get levels */
    getLevels: () => levels.get(),

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
        log.debug("post update:");
        LEVELS.print();
        return true;
      } catch (e) {
        log.debug(e);
        return false;
      }
    },

    print: () =>
      levels.levels.forEach(level =>
        log.debug(
          `difficulty_factor: ${level.difficulty_factor} visitor ${level.visitor_threshold}`,
        ),
      ),

    /** remove level */
    remove: (id: number) => {
      log.debug(`[LEVELS] received order to remove ${id} element`);

      const tmpLevel = new Levels();

      id -= 1;
      try {
        for (let i = 0; i < levels.levels.length; i++) {
          if (id != i) {
            tmpLevel.add(levels.levels[i]);
          } else {
            log.debug(`[LEVELS] removing ${i} element`);
            const rmElement = levels.levels[i];
            log.debug(
              `[LEVELS] removing element: 
              difficulty_factor: ${rmElement.difficulty_factor} 
              visitor_threshold: ${rmElement.visitor_threshold}`,
            );
          }
        }
        levels.levels = tmpLevel.levels;
        log.debug("Post remove:");
        LEVELS.print();
        return true;
      } catch (e) {
        log.debug(e);
        return false;
      }
    },
  };
})();
