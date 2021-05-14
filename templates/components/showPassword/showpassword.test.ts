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

import registerShowPassword from './index';
import {showPassword} from './index';

const initial_content = `
<form class="sitekey-form" method="POST" action="/api/v1/signin" id="form" data-bitwarden-watching="1">
    <h1 class="form__title">
      Signin to mCaptcha
    </h1>
    <label class="sitekey-form__label" for="username">
      Username
      <input class="sitekey-form__input" type="text" name="username" id="username" required="" data-com.bitwarden.browser.user-edited="yes">
    </label>

    <label class="sitekey-form__label" for="duration">
      Password
      <input class="sitekey-form__input" type="password" name="password" id="password" required="">
		  <span class="show-password-container">
  <img class="show-password--show" src="/static/img/svg/eye.svg" alt="">
  <img class="show-password--hide" src="/static/img/svg/eye-off.svg" alt="">
</span>
	</label>

    <input type="submit" class="sitekey-form__submit" value="Sign in">
  </form>
  `;

it('show password works', () => {
  document.body.innerHTML = initial_content;

  const container = <HTMLElement>(
    document.querySelector(`.show-password-container`)
  );
  const hide = <HTMLElement>container.querySelector('.show-password--hide');
  const show = <HTMLElement>container.querySelector('.show-password--show');
  const password = <HTMLInputElement>document.getElementById('password');
  show.style.display = 'inline';
  hide.style.display = 'none';

  showPassword();
  expect(hide.style.display).toEqual('inline');
  expect(show.style.display).toEqual('none');
  expect(password.type).toEqual('text');

  showPassword();
  expect(show.style.display).toEqual('inline');
  expect(hide.style.display).toEqual('none');
  expect(password.type).toEqual('password');
});

it('show password click works', () => {
  document.body.innerHTML = initial_content;

  const container = <HTMLElement>(
    document.querySelector(`.show-password-container`)
  );
  const hide = <HTMLElement>container.querySelector('.show-password--hide');
  const show = <HTMLElement>container.querySelector('.show-password--show');
  const password = <HTMLInputElement>document.getElementById('password');
  show.style.display = 'inline';
  hide.style.display = 'none';

  registerShowPassword();
  container.click();
  expect(hide.style.display).toEqual('inline');
  expect(show.style.display).toEqual('none');
  expect(password.type).toEqual('text');

  container.click();
  expect(show.style.display).toEqual('inline');
  expect(hide.style.display).toEqual('none');
  expect(password.type).toEqual('password');
});
