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

/** get login form HTML */
export const getLoginFormHtml = () =>
  `
  <form method="POST" action="/something" id="form">
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

/** get registration form HTML */
export const getRegistrationFormHtml = () => `
  <form method="POST" action="/api/v1/signup"  class="form__box" id="form">
    <label class="form__in-group" for="username"
      >Username
      <input
        class="form__in-field"
        id="username"
        type="text"
        name="username"
        id="username"
        required
        autofocus="true"
      />
    </label>

    <label class="form__in-group" for="username"
      >Email(optional)
      <input
        class="form__in-field"
        id="email"
        type="email"
        name="email"
        id="email"
      />
    </label>

    <label for="password" class="form__in-group"
      >Password
      <input
        class="form__in-field"
        type="password"
        id="password"
        name="password"
        id="password"
        required
      />
    </label>

    <label for="password" class="form__in-group"
      >Re-enter Password
      <input
        class="form__in-field"
        type="password"
        id="password-check"
        name="password-check"
        id="password-check"
        required
      />
    </label>
        <input type="submit" class="form__submit-button" value="Sign up" />
  </form>
`;

export const mockAlert = () => {
  delete window.alert;

  window.alert = (x: any) => console.log(x);
};
