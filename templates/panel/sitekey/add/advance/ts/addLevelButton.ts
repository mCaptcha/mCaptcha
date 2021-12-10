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
import validateLevel from "./levels/validateLevel";
import getNumLevels from "./levels/getNumLevels";
import * as UpdateLevel from "./levels/updateLevel";
import {
  getRemoveButtonHTML,
  addRemoveLevelButtonEventListener,
} from "./removeLevelButton";
import CONST from "./const";

import log from "../../../../../logger";

/**
 * Gets executed when 'Add' Button is clicked to add levels
 * Used to validate levels per m_captcha::defense::Defense's
 * specifications
 */
const addLevel = (e: Event) => {
  const eventTarget = <HTMLElement>e.target;
  const PARENT = <HTMLLabelElement>eventTarget.parentElement;
  const FIELDSET = <HTMLElement>PARENT.parentElement;
  const onScreenLevel = getNumLevels();

  const isValid = validateLevel(onScreenLevel);
  log.debug(`[addLevelButton] isValid: ${isValid}`);
  if (!isValid) {
    const error = `Aborting level ${onScreenLevel} addition`;
    return log.error(error);
  }

  FIELDSET.replaceChild(getRemoveButtonHTML(onScreenLevel), PARENT);

  const newLevelElement = getHtml(onScreenLevel + 1);
  FIELDSET.insertAdjacentElement("afterend", newLevelElement);
  UpdateLevel.register(onScreenLevel);

  addRemoveLevelButtonEventListener(onScreenLevel);
  addLevelButtonAddEventListener();
  const main = document.querySelector("body");
  const style = main.style.display;
  main.style.display = "none";
  main.style.display = style;
};

/** adds onclick event listener */
const addLevelButtonAddEventListener = (): void => {
  const addLevelButton = <HTMLElement>(
    document.querySelector(`.${CONST.ADD_LEVEL_BUTTON}`)
  );
  addLevelButton.addEventListener("click", addLevel);
};

/**
 * Generate HTML to be added when 'Add Level' button is clicked
 * Check if './add-level.html` to see if this is up to date
 */
const getHtml = (level: number) => {
  log.debug(`[generating HTML getHtml]level: ${level}`);

  const fieldset = document.createElement("fieldset"); //  new HTMLFieldSetElement();
  fieldset.className = CONST.LEVEL_CONTAINER_CLASS;
  fieldset.id = `${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${level}`;

  const legend = document.createElement("legend"); // new HTMLLegendElement();
  legend.className = CONST.LEGEND_CLASS;
  const legendText = document.createTextNode(`Level ${level}`);
  legend.appendChild(legendText);

  fieldset.appendChild(legend);

  const vistitorLabel = document.createElement("label"); //document.createElement('label');
  vistitorLabel.className = CONST.LABEL_CLASS;
  const visitorText = document.createTextNode("Visitor");
  vistitorLabel.appendChild(visitorText);
  const visitor = document.createElement("input"); //document.createElement('input');
  const visitorId = `${CONST.VISITOR_WITHOUT_LEVEL}${level}`;
  visitor.className = CONST.LEVEL_INPUT_CLASS;
  visitor.type = "number";
  visitor.name = visitorId;
  visitor.id = visitorId;
  vistitorLabel.htmlFor = visitorId;
  vistitorLabel.appendChild(visitor);

  fieldset.appendChild(vistitorLabel);

  const difficultyLabel = document.createElement("label");
  difficultyLabel.className = CONST.LABEL_CLASS;
  const difficultyText = document.createTextNode("Difficulty");
  difficultyLabel.appendChild(difficultyText);
  const difficulty = document.createElement("input");
  const difficultyID = `${CONST.DIFFICULTY_WITHOUT_LEVEL}${level}`;
  difficulty.type = "number";
  difficulty.name = difficultyID;
  difficulty.className = CONST.LEVEL_INPUT_CLASS;
  difficulty.id = difficultyID;
  difficultyLabel.htmlFor = difficultyID;
  difficultyLabel.appendChild(difficulty);

  fieldset.appendChild(difficultyLabel);

  const addLevelLabel = document.createElement("label");
  addLevelLabel.className = CONST.REMOVE_LEVEL_LABEL_CLASS;
  const addLevel = document.createElement("input");
  addLevel.className = CONST.ADD_LEVEL_BUTTON;
  addLevel.type = "button";
  const addLevelButtonID = "add";
  addLevel.name = addLevelButtonID;
  addLevel.id = addLevelButtonID;
  addLevelLabel.htmlFor = addLevelButtonID;
  const addText = document.createTextNode("Add level");
  addLevelLabel.appendChild(addText);
  addLevel.value = "Add";
  addLevelLabel.appendChild(addLevel);

  fieldset.appendChild(addLevelLabel);

  return fieldset;
};

export default addLevelButtonAddEventListener;
