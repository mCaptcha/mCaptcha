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

import ROUTES from '../../api/v1/routes';
import VIEWS from '../../views/v1/routes';

import isBlankString from '../../utils/isBlankString';
import genJsonPayload from '../../utils/genJsonPayload';

import userExists from './userExists';
import {checkEmailExists} from './emailExists';

//import '../forms.scss';

const usernameElement = <HTMLInputElement>document.getElementById('username');
const emailElement = <HTMLInputElement>document.getElementById('email');
const passwordElement = <HTMLInputElement>document.getElementById('password');

const registerUser = async (e: Event) => {
  e.preventDefault();

  let username = usernameElement.value;
  isBlankString(e, username, 'username');
  //isBlankString(e);//, username, 'username');

  let password = passwordElement.value;
  const passwordCheckElement = <HTMLInputElement>(
    document.getElementById('password-check')
  );
  let passwordCheck = passwordCheckElement.value;
  if (password != passwordCheck) {
    return alert("passwords don't match, check again!");
  }

  let exists = await userExists();
  if (exists) {
    return;
  }

  let email: string|null = emailElement.value;
  if (!email.replace(/\s/g, '').length) {
    email = null;
  } else {
    exists = await checkEmailExists();
    if (exists) {
      return;
    }
  }

  let payload = {
    username,
    password,
    email,
  };

  let res = await fetch(ROUTES.registerUser, genJsonPayload(payload));
  if (res.ok) {
    alert('success');
    window.location.assign(VIEWS.loginUser);
  } else {
    let err = await res.json();
    alert(`error: ${err.error}`);
  }
};

export const index = () => {
  let form = <HTMLFontElement>document.getElementById('form');
  form.addEventListener('submit', registerUser, true);
  usernameElement.addEventListener('input', userExists, false);
};
