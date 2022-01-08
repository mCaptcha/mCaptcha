/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

import { getPassword } from "../../../auth/login/ts/";
import FORM from "../../../auth/sudo/";
import additionalData from "../../../components/additional-data";
import registerShowPassword from "../../../components/showPassword";

import getFormUrl from "../../../utils/getFormUrl";
import genJsonPayload from "../../../utils/genJsonPayload";
import createError from "../../../components/error";

import VIEWS from "../../../views/v1/routes";

const submit = async (e: Event) => {
  e.preventDefault();
  const password = getPassword();
  const key = additionalData().dataset.sitekey;

  const payload = {
    password,
    key,
  };

  const formUrl = getFormUrl(<HTMLFormElement>FORM.get());

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    window.location.assign(VIEWS.listSitekey);
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

export const index = (): void => {
  FORM.get().addEventListener("submit", submit, true);
  registerShowPassword();
};
