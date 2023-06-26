// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
