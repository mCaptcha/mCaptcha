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

import getFormUrl from './getFormUrl';

'use strict';

const formClassName = 'form__box';
const formURL = '/api/v1/signin';

document.body.innerHTML = `
  <form method="POST" action="${formURL}" class="${formClassName}" id="form">
    <label class="form__in-group" for="username"
      >Username
      <input
        class="form__in-field"
        id="username"
        type="text"
        name="username"
        required=""
        autofocus="true"
      />
    </label>

    <label for="password" class="form__in-group"
      >Password
      <input
        class="form__in-field"
        type="password"
        id="password"
        name="password"
        required=""
      />
      <!--
          <a class="form__pw-recovery" -href="/recovert/password"
            >Forgot password?</a
          >
          -->
    </label>
        <input type="submit" class="form__submit-button" value="Sign in" />
  </form>
`;

it('getFromUrl workds', () => {
  const name = `.${formClassName}`;
  expect(getFormUrl(name)).toContain(formURL);

  const form = <HTMLFormElement>document.querySelector('form');
  expect(getFormUrl(form)).toContain(formURL);

  expect(getFormUrl()).toContain(formURL);
});
