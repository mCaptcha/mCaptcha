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

import genJsonPayload from '../../utils/genJsonPayload';

const checkEmailExists = async () => {
  let email = document.getElementById('email');
  let val = email.value;
  let payload = {
    val,
  };

  //  return fetch(ROUTES.emailExists, genJsonPayload(payload)).then(res => {
  //    if (res.ok) {
  //      res.json().then(data => {
  //        if (data.exists) {
  //          console.log(email.className);
  //          email.className += ' form__in-field--warn';
  //          alert('Email taken');
  //        }
  //
  //        return data.exists;
  //      });
  //    } else {
  //      res.json().then(err => alert(`error: ${err.error}`));
  //    }
  //  });
  //

  let res = await fetch(ROUTES.emailExists, genJsonPayload(payload));
  if (res.ok) {
    let data = await res.json();
    if (data.exists) {
      email.className += ' form__in-field--warn';
      alert('Email taken');
    }
    return data.exists;
  } else {
    let err = await res.json();
    alert(`error: ${err.error}`);
  }
};

export {checkEmailExists};
