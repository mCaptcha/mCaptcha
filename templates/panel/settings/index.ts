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

import registerShowPassword from '../../components/showPassword/';
import CopyIcon from '../../components/clipboard/';
import createError from '../../components/error/';

import emailExists from '../../auth/register/ts/emailExists';
import userExists from '../../auth/register/ts/userExists';

import LazyElement from '../../utils/lazyElement';
import isBlankString from '../../utils/isBlankString';
import getFormUrl from '../../utils/getFormUrl';
import genJsonPayload from '../../utils/genJsonPayload';

import VIEWS from '../../views/v1/routes';

const SECRET_COPY_ICON = 'settings__secret-copy';
const SECRET_COPY_DONE_ICON = 'settings__secret-copy-done';

// form IDs
const DELETE_FORM = 'settings__delete-form';
const EMAIL_FORM = 'settings__email-form';
const USERNAME_FORM = 'settings__username-form';
const SECRET_FORM = 'settings__secret-form';

// form elements
const deleteForm = new LazyElement(DELETE_FORM);
const emailForm = new LazyElement(EMAIL_FORM);
const usernameForm = new LazyElement(USERNAME_FORM);
const secretForm = new LazyElement(SECRET_FORM);

// field IDs
const EMAIL = 'email';
const USERNAME = 'username';

// field elements
const emailField = new LazyElement(EMAIL);
const usernameField = new LazyElement(USERNAME);

// form event handlers
const updateEmail = async (e: Event) => {
  e.preventDefault();
  const emailElement = <HTMLInputElement>emailField.get();
  const email = emailElement.value;
  isBlankString(email, 'email', e);
  if (await emailExists(emailElement)) {
    return;
  } else {
    const url = getFormUrl(<HTMLFormElement>emailForm.get());
    const payload = {
      email,
    };

    const res = await fetch(url, genJsonPayload(payload));
    if (res.ok) {
      window.location.reload();
    } else {
      const err = await res.json();
      createError(err.error);
    }
  }
};

const updateUsername = async (e: Event) => {
  e.preventDefault();
  const usernameElement = <HTMLInputElement>usernameField.get();
  const username = usernameElement.value;
  isBlankString(username, 'username', e);
  if (await userExists(usernameElement)) {
    return;
  } else {
    const url = getFormUrl(<HTMLFormElement>usernameForm.get());
    const payload = {
      username,
    };
    const res = await fetch(url, genJsonPayload(payload));
    if (res.ok) {
      window.location.reload();
    } else {
      const err = await res.json();
      createError(err.error);
    }
  }
};

const updateSecret = (e: Event) => {
  e.preventDefault();
  const msg =
    'WARNING: updating secret will cause service disruption if old secret is still in use post update';
  if (confirm(msg)) {
    window.location.assign(VIEWS.updateSecret);
  }
};

const deleteAccount = (e: Event) => {
  e.preventDefault();
  const msg =
    "WARNING: all CAPTCHA configurations will be lost. This action can't be undone";
  if (confirm(msg)) {
    window.location.assign(VIEWS.deleteAccount);
  }
};

// regist form event handlers
const registerForms = () => {
  deleteForm.get().addEventListener('submit', e => deleteAccount(e), true);
  emailForm.get().addEventListener('submit', e => updateEmail(e), true);
  usernameForm.get().addEventListener('submit', e => updateUsername(e), true);
  console.log(usernameField.get());
  usernameField
    .get()
    .addEventListener('input', async () => await userExists(), false);
  secretForm.get().addEventListener('submit', e => updateSecret(e), true);
};

// set up copying account secret to clipboard
const initCopySecret = () => {
  const secretElement = <HTMLElement>(
    document.querySelector(`.${SECRET_COPY_ICON}`)
  );
  const writeText = secretElement.dataset.secret;
  new CopyIcon(writeText, secretElement, SECRET_COPY_DONE_ICON);
};

/// TODO email update button should only change if email value has been changed
const index = () => {
  registerShowPassword();
  initCopySecret();
  registerForms();
};

export default index;
