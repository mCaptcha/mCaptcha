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

import isBlankString from '../../utils/genJsonPayload';
import genJsonPayload from '../../utils/genJsonPayload';

import userExists from './userExists';
import {checkEmailExists} from './emailExists';

//import '../forms.scss';

const registerUser = async e => {
  e.preventDefault();

  let username = document.getElementById('username').value;
  isBlankString(e, username, 'username');

  let password = document.getElementById('password').value;
  let passwordCheck = document.getElementById('password-check').value;
  if (password != passwordCheck) {
    return alert("passwords don't match, check again!");
  }

  let exists = await userExists();
  if (exists) {
    return;
  }

  let email = document.getElementById('email').value;
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
  let form = document.getElementById('form');
  form.addEventListener('submit', registerUser, true);

  let username = document.getElementById('username');
  username.addEventListener('input', userExists, false);
};
