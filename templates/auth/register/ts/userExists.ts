// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import ROUTES from "../../../api/v1/routes";

import genJsonPayload from "../../../utils/genJsonPayload";
import createError from "../../../components/error/index";

const userExists = async (element?: HTMLInputElement): Promise<boolean> => {
  console.log(element);
  let username;
  if (element === undefined) {
    username = <HTMLInputElement>document.getElementById("username");
  } else {
    username = element;
  }
  const val = username.value;
  const payload = {
    val,
  };

  const res = await fetch(ROUTES.usernameExists, genJsonPayload(payload));
  if (res.ok) {
    const data = await res.json();
    if (data.exists) {
      username.className += " form__in-field--warn";
      createError(`Username "${val}" taken`);
    }
    return data.exists;
  } else {
    const err = await res.json();
    createError(err.error);
  }
  return false;
};

export default userExists;
