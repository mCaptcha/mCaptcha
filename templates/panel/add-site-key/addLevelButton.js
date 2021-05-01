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

const LABEL_CONTAINER_CLASS = 'sitekey-form__add-level-flex-container';
const ADD_LEVEL_BUTTON = 'sitekey-form__add-level-button';
const LABEL_CLASS = 'sitekey-form__label';
const INPUT_ID_WITHOUT_LEVEL = 'level';
const LABEL_INNER_TEXT_WITHOUT_LEVEL = 'Level ';
const INPUT_CLASS = 'sitekey-form__input--add-level';
const ADD_LEVEL_BUTTON_INNER_TEXT = 'Add Level';

const addLevelButtonEventHandler = e => {
  const PREV_LEVEL_CONTAINER = e.target.parentElement;
  e.target.remove();

  let numLevels = 0;
  document.querySelectorAll(`.${LABEL_CLASS}`).forEach(_ => numLevels++);
  numLevels = numLevels.toString();

  let labelContainer = document.createElement('div');
  labelContainer.className = LABEL_CONTAINER_CLASS;

  let inputID = INPUT_ID_WITHOUT_LEVEL + numLevels;
  let label = document.createElement('label');
  label.className = LABEL_CLASS;
  label.htmlFor = inputID;
  label.innerText = LABEL_INNER_TEXT_WITHOUT_LEVEL + numLevels;

  labelContainer.appendChild(label);

  PREV_LEVEL_CONTAINER.insertAdjacentElement('afterend', labelContainer);

  let inputContainer = document.createElement('div');
  inputContainer.className = LABEL_CONTAINER_CLASS;

  let input = document.createElement('input');
  input.id = inputID;
  input.name = inputID;
  input.type = 'text';
  input.className = INPUT_CLASS;

  inputContainer.appendChild(input);

  let button = document.createElement('button');
  button.className = ADD_LEVEL_BUTTON;
  button.innerText = ADD_LEVEL_BUTTON_INNER_TEXT;

  inputContainer.appendChild(button);

  labelContainer.insertAdjacentElement('afterend', inputContainer);


  addLevelButtonAddEventListener();
};

export const addLevelButtonAddEventListener = () => {
  let addLevelButton = document.querySelector(`.${ADD_LEVEL_BUTTON}`);
  addLevelButton.addEventListener('click', addLevelButtonEventHandler);
};

/*
 <div class="sitekey-form__add-level-flex-container">
<label class="sitekey-form__label" for="level2">Level 2</label>
</div>

<div class="sitekey-form__add-level-flex-container">
<input
  class="sitekey-form__input--add-level"
  type="text"
  name="level2"
  id="level2"
  value=""
/>
<button class="sitekey-form__add-level-button">Add Level</button>
</div>
*/
