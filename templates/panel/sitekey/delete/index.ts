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

import {getPassword} from '../../../auth/login/ts/';
import form from '../../../auth/sudo/';
import additionalData from '../../../components/additional-data';

import getFormUrl from '../../../utils/getFormUrl';
import genJsonPayload from '../../../utils/genJsonPayload';
import createError from '../../../components/error';

import VIEWS from '../../../views/v1/routes';

const submit = async (e: Event) => {
  e.preventDefault();
  const password = getPassword();
  const key = additionalData().dataset.sitekey;

  const payload = {
    password,
    key,
  };

  const formUrl = getFormUrl(form());

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    window.location.assign(VIEWS.listSitekey);
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

export const index = () => {
  form().addEventListener('submit', submit, true);
};
