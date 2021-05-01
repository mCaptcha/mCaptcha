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

const VALIDATE_LEVELS = (function() {
  const levels: Array<number> = [];

  const checkAscendingOrder = (newLevel: number) => {
    if (levels.length == 0) {
      return true;
    }

    let isValid = true;
    levels.find(level => {
      if (level > newLevel) {
        alert(
          `Level: ${newLevel} has to greater than previous levels ${level}`,
        );
        isValid = false;
        return true;
      }
      return false;
    });

    return isValid;
  };
  return {
    add: function(newLevel: number) {
      console.log(`[levels.js]levels: ${levels} newLevel: ${newLevel}`);
      if (levels.find(level => level == newLevel)) {
        alert(`Level: ${newLevel} has to be unique`);
        return false;
      }

      let isValid = checkAscendingOrder(newLevel);
      if (isValid) {
        levels.push(newLevel);
        return true;
      }

      console.log(
        `Ascending arder failure. Levels: ${levels}, levels length: ${levels.length}`,
      );
      return false;
    },
  }; })();

export default VALIDATE_LEVELS;
