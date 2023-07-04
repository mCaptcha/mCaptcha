// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//import {init} from "mcaptcha-glue";

import VIEWS from "../../../views/v1/routes";

import isBlankString from "../../../utils/isBlankString";
import genJsonPayload from "../../../utils/genJsonPayload";
import getFormUrl from "../../../utils/getFormUrl";
import registerShowPassword from "../../../components/showPassword";
import createError from "../../../components/error/index";

//import '../forms.scss';

export const getPassword = (): string | null => {
  const passwordElement = <HTMLInputElement>document.getElementById("password");
  if (passwordElement === null) {
    console.debug("Password is null");
    return;
  }

  return passwordElement.value;
};

const login = async (e: Event): Promise<void> => {
  e.preventDefault();
  const loginElement = <HTMLInputElement>document.getElementById("login");
  if (loginElement === null) {
    console.debug("login element element is null");
    return;
  }

  const login = loginElement.value;
  isBlankString(login, "username", e);

  const password = getPassword();

  const payload = {
    login,
    password,
  };

  const formUrl = getFormUrl();

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    window.location.assign(VIEWS.panelHome);
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

export const index = (): void => {
  const form = <HTMLFontElement>document.getElementById("form");
  form.addEventListener("submit", login, true);
  registerShowPassword();
//  init();
};
