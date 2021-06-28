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

import fetchMock from 'jest-fetch-mock';

import userExists from './userExists';

import {mockAlert, getLoginFormHtml} from '../../../setUpTests';

import setup from '../../../components/error/setUpTests';

fetchMock.enableMocks();
mockAlert();

beforeEach(() => {
  fetchMock.resetMocks();
});

it('finds exchange', async () => {
  fetchMock.mockResponseOnce(JSON.stringify({exists: true}));

  document.body.innerHTML = getLoginFormHtml();
  document.querySelector('body').appendChild(setup());
  const usernameField = <HTMLInputElement>document.querySelector('#username');
  usernameField.value = 'test';
  expect(await userExists()).toBe(true);

  fetchMock.mockResponseOnce(JSON.stringify({exists: false}));
  expect(await userExists()).toBe(false);
});
