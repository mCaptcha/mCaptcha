// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import registerShowPassword from "./index";
import {showPassword} from "./index";

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

it("show password works", () => {
  document.body.innerHTML = initial_content;

  const container = <HTMLElement>(
    document.querySelector(".show-password-container")
  );
  const hide = <HTMLElement>container.querySelector(".show-password--hide");
  const show = <HTMLElement>container.querySelector(".show-password--show");
  const password = <HTMLInputElement>document.getElementById("password");
  show.style.display = "inline";
  hide.style.display = "none";

  showPassword();
  expect(hide.style.display).toEqual("inline");
  expect(show.style.display).toEqual("none");
  expect(password.type).toEqual("text");

  showPassword();
  expect(show.style.display).toEqual("inline");
  expect(hide.style.display).toEqual("none");
  expect(password.type).toEqual("password");
});

it("show password click works", () => {
  document.body.innerHTML = initial_content;

  const container = <HTMLElement>(
    document.querySelector(".show-password-container")
  );
  const hide = <HTMLElement>container.querySelector(".show-password--hide");
  const show = <HTMLElement>container.querySelector(".show-password--show");
  const password = <HTMLInputElement>document.getElementById("password");
  show.style.display = "inline";
  hide.style.display = "none";

  registerShowPassword();
  container.click();
  expect(hide.style.display).toEqual("inline");
  expect(show.style.display).toEqual("none");
  expect(password.type).toEqual("text");

  container.click();
  expect(show.style.display).toEqual("inline");
  expect(hide.style.display).toEqual("none");
  expect(password.type).toEqual("password");
});
