// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/** get login form HTML */
export const getLoginFormHtml = (): string =>
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
export const getRegistrationFormHtml = (): string => `
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

export const mockAlert = (): void => {
  delete window.alert;

  window.alert = (x: any) => console.log(x);
};
