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

/** Conditional logger singleton */
const log = (function() {
  let mode = MODE.production;
  return {
    /** console.error() wrapper */
    debug: (data: any) => {
      if (mode == MODE.none) {
        return;
      }
      if (mode == MODE.development) {
        console.debug(data);
      }
    },

    /** console.error() wrapper */
    error: (data: any) => {
      console.error(data);
    },

    /** console.log() wrapper */
    log: (data: any) => {
      if (mode == MODE.none) {
        return;
      }

      console.log(data);
    },

    /** set logging mode */
    setMode: (newMode: MODE) => (mode = newMode),
  };
})();

export const enum MODE {
  production,
  development,
  none,
}

export default log;
