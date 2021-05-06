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

import {Router} from './router';

'use strict';

const result = {
  result: '',
};

const panelResult = 'hello from panel';
const panelRoute = '/panel';
const panel = () => (result.result = panelResult);

const settingsRoute = '/settings/';
const settingsResult = 'hello from settings';
const settings = () => (result.result = settingsResult);

const UriExistsErr = 'URI exists';
const emptyUriErr = 'uri is empty';
const unregisteredRouteErr = "Route isn't registered";

const router = new Router();
router.register(panelRoute, panel);
router.register(settingsRoute, settings);

it('checks if Router works', () => {
  window.history.pushState({}, 'Settings', settingsRoute);
  router.route();
  expect(result.result).toBe(settingsResult);
  window.history.pushState({}, 'Panel', panelRoute);
  router.route();
  expect(result.result).toBe(panelResult);

  // duplicate URI registration
  try {
    router.register(settingsRoute, settings);
  } catch (e) {
    expect(e.message).toBe(UriExistsErr);
  }

  // empty URI registration
  try {
    router.register('      ', settings);
  } catch (e) {
    expect(e.message).toBe(emptyUriErr);
  }

  // routing to unregistered route
  try {
    window.history.pushState({}, `Page Doesn't Exist`, `/page/doesnt/exist`);
    router.route();
  } catch (e) {
    expect(e.message).toBe(unregisteredRouteErr);
  }
});
