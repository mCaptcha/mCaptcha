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

import CONST from './const';
import getNumLevels from './levels/getNumLevels';
import isBlankString from '../../utils/isBlankString';
import getFormUrl from '../../utils/getFormUrl';
import genJsonPayload from '../../utils/genJsonPayload';
import {LEVELS} from './levels';

import VIEWS from '../../views/v1/routes';

const SITE_KEY_FORM_CLASS = 'sitekey-form';
const FORM = <HTMLFormElement>document.querySelector(`.${SITE_KEY_FORM_CLASS}`);
//const FORM_SUBMIT_BUTTON_CLASS = "sitekey-form__submit";
//const FORM_SUBMIT_BUTTON = <HTMLButtonElement>document.querySelector(`.${FORM_SUBMIT_BUTTON_CLASS}`);

const addSubmitEventListener = () => {
  FORM.addEventListener('submit', submit, true);
};

//const validateLevels = (e: Event) => {
//  const numLevels = getNumLevels();
//  // check if levels are unique and are in increasing order;
//  // also if they are positive
//  // also if level input field is accompanied by a "Add Level" button,
//  // it shouldn't be used for validation
//  for (let levelNum = 1; levelNum < numLevels; levelNum++) {
//    const inputID = CONST.INPUT_ID_WITHOUT_LEVEL + levelNum;
//    const inputElement = <HTMLInputElement>document.getElementById(inputID);
//    const val = inputElement.value;
//    const filed = CONST.LABEL_INNER_TEXT_WITHOUT_LEVEL + levelNum;
//    isBlankString(val, filed, e);
//  }
//};

const validateDescription = (e: Event) => {
  const inputElement = <HTMLInputElement>document.getElementById('description');
  const val = inputElement.value;
  const filed = 'Description';
  isBlankString(val, filed, e);
};

const submit = async (e: Event) => {
  e.preventDefault();

  validateDescription(e);
//  validateLevels(e);

  const formUrl = getFormUrl(FORM);

  const levels = LEVELS.getLevels();
  console.debug(`[form submition]: levels: ${levels}`);

  const payload = {
    levels: levels,
  };

  console.debug(`[form submition] json payload: ${JSON.stringify(payload)}`);

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    alert('success');
    window.location.assign(VIEWS.sitekey);
  } else {
    const err = await res.json();
    alert(`error: ${err.error}`);
  }
};

export default addSubmitEventListener;
