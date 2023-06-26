// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import VIEWS from "../../../views/v1/routes";

import isBlankString from "../../../utils/isBlankString";
import genJsonPayload from "../../../utils/genJsonPayload";

import userExists from "./userExists";
import emailExists from "./emailExists";
import getFormUrl from "../../../utils/getFormUrl";
import registerShowPassword from "../../../components/showPassword";
import createError from "../../../components/error/index";

//import '../forms.scss';

const usernameElement = <HTMLInputElement>document.getElementById("username");
const emailElement = <HTMLInputElement>document.getElementById("email");
const passwordElement = <HTMLInputElement>document.getElementById("password");

const registerUser = async (e: Event): Promise<void> => {
  e.preventDefault();

  const username = usernameElement.value;
  isBlankString(username, "username", e);
  //isBlankString(e);//, username, 'username');

  const password = passwordElement.value;
  const passwordCheckElement = <HTMLInputElement>(
    document.getElementById("password-check")
  );
  const passwordCheck = passwordCheckElement.value;
  if (password != passwordCheck) {
    return createError("passwords don't match, check again!");
  }

  let exists = await userExists();
  if (exists) {
    return;
  }

  let email: string | null = emailElement.value;
  if (!email.replace(/\s/g, "").length) {
    email = null;
  } else {
    exists = await emailExists();
    if (exists) {
      return;
    }
  }

  const payload = {
    username,
    password,
    confirm_password: passwordCheck,
    email,
  };
  const formUrl = getFormUrl();

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    window.location.assign(VIEWS.loginUser);
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

export const index = (): void => {
  const form = <HTMLFontElement>document.getElementById("form");
  form.addEventListener("submit", registerUser, true);
  usernameElement.addEventListener(
    "input",
    async () => await userExists(),
    false,
  );
  registerShowPassword();
};
