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

const SITE_KEY_FORM_CLASS = 'sitekey-form';
const FORM = <HTMLFormElement>document.querySelector(`.${SITE_KEY_FORM_CLASS}`);

import * as addLevelButton from './addLevelButton';
import isBlankString from '../../utils/isBlankString';

export const addSubmitEventListener = () => {
  FORM.addEventListener('submit', submit, true);
};

const validateLevels = (e: Event) => {
  let numLevels = addLevelButton.getNumLevels();
  // check if levels are unique and are in increasing order;
  // also if they are positive
  // also if level input field is accompanied by a "Add Level" button,
  // it shouldn't be used for validation
  for (let levelNum = 1; levelNum < numLevels; levelNum++) {
    let inputID = addLevelButton.INPUT_ID_WITHOUT_LEVEL + levelNum;
    let inputElement = <HTMLInputElement>document.getElementById(inputID);
    let val = inputElement.value;
    let filed = addLevelButton.LABEL_INNER_TEXT_WITHOUT_LEVEL + levelNum;
    isBlankString(e, val, filed);
  }
}

const validateDescription = (e: Event) => {
    let inputElement = <HTMLInputElement>document.getElementById("description");
    let val = inputElement.value;
    let filed = "Description";
    isBlankString(e, val, filed);
}

const submit = async (e: Event) => {
  validateDescription(e);
  validateLevels(e);
  // get values
  // check validate levels
  // submit
  // handle erros
};
