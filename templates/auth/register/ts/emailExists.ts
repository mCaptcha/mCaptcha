// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import ROUTES from "../../../api/v1/routes";

import genJsonPayload from "../../../utils/genJsonPayload";
import createError from "../../../components/error/index";

const emailExists = async (element?: HTMLInputElement): Promise<boolean> => {
  let email;
  if (element === undefined || element === null) {
    email = <HTMLInputElement>document.getElementById("email");
  } else {
    email = element;
  }
  const val = email.value;

  const payload = {
    val,
  };

  const res = await fetch(ROUTES.emailExists, genJsonPayload(payload));
  if (res.ok) {
    const data = await res.json();
    if (data.exists) {
      email.className += " form__in-field--warn";
      createError(`Email "${val}" is already used`);
      return data.exists;
    }
    return data.exists;
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

export default emailExists;
