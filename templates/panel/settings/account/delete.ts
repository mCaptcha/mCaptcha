// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {getPassword} from "../../../auth/login/ts/";
import FORM from "../../../auth/sudo/";

import getFormUrl from "../../../utils/getFormUrl";
import genJsonPayload from "../../../utils/genJsonPayload";
import createError from "../../../components/error";
import registerShowPassword from "../../../components/showPassword";

import VIEWS from "../../../views/v1/routes";

const submit = async (e: Event) => {
  e.preventDefault();
  const password = getPassword();

  const payload = {
    password,
  };

  const formUrl = getFormUrl(<HTMLFormElement>FORM.get());

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    window.location.assign(VIEWS.panelHome);
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

export const index = (): void => {
  FORM.get().addEventListener("submit", submit, true);
  registerShowPassword();
};
