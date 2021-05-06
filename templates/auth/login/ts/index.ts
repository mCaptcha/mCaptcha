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
import getFormUrl from '../../../utils/getFormUrl';

//import '../forms.scss';

const login = async (e: Event) => {
  e.preventDefault();
  const usernameElement = <HTMLInputElement>document.getElementById('username');
  if (usernameElement === null) {
    console.debug('Username element is null');
    return;
  }

  const username = usernameElement.value;
  isBlankString(username, 'username', e);

  const passwordElement = <HTMLInputElement>document.getElementById('password');
  if (passwordElement === null) {
    console.debug('Password is null');
    return;
  }

  const password = passwordElement.value;

  const payload = {
    username,
    password,
  };

  const formUrl = getFormUrl(null);

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    alert('success');
    window.location.assign(VIEWS.panelHome);
  } else {
    const err = await res.json();
    alert(`error: ${err.error}`);
  }
};

export const index = () => {
  const form = <HTMLFontElement>document.getElementById('form');
  form.addEventListener('submit', login, true);
};
