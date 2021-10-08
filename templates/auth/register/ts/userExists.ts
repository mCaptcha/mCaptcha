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
