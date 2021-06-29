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

import VIEWS from '../../../views/v1/routes';

import isBlankString from '../../../utils/isBlankString';
import genJsonPayload from '../../../utils/genJsonPayload';

import userExists from './userExists';
import emailExists from './emailExists';
import getFormUrl from '../../../utils/getFormUrl';
import registerShowPassword from '../../../components/showPassword';
import createError from '../../../components/error/index';

//import '../forms.scss';

const usernameElement = <HTMLInputElement>document.getElementById('username');
const emailElement = <HTMLInputElement>document.getElementById('email');
const passwordElement = <HTMLInputElement>document.getElementById('password');

const registerUser = async (e: Event) => {
  e.preventDefault();

  const username = usernameElement.value;
  isBlankString(username, 'username', e);
  //isBlankString(e);//, username, 'username');

  const password = passwordElement.value;
  const passwordCheckElement = <HTMLInputElement>(
    document.getElementById('password-check')
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
  if (!email.replace(/\s/g, '').length) {
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

export const index = () => {
  const form = <HTMLFontElement>document.getElementById('form');
  form.addEventListener('submit', registerUser, true);
  usernameElement.addEventListener('input', userExists, false);
  registerShowPassword();
};
