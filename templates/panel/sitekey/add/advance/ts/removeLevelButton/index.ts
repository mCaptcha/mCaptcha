// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { LEVELS } from "../levels/index";
import updateLevelNumbersOnDOM from "./updateDom";
import CONST from "../const";

import log from "../../../../../../logger";

const REMOVE_LEVEL_BUTTON = "sitekey-form__level-remove-level-button";

/**
 * Gets executed when 'Remove' Button is clicked to remove levels
 */
const removeLevel = (e: Event) => {
  const eventTarget = <HTMLElement>e.target;
  const PARENT = <HTMLElement>eventTarget.parentElement;
  const FIELDSET = <HTMLElement>PARENT.parentElement;

  const levelNum = parseInt(
    eventTarget.id.slice(CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL.length)
  );

  if (Number.isNaN(levelNum)) {
    const msg =
      "[removeLevelButton.ts] error in parsing level number from remove button ID";
    //log.error(msg);
    throw new Error(msg);
  }
  updateLevelNumbersOnDOM(levelNum);

  LEVELS.remove(levelNum);
  FIELDSET.remove();
};

/** adds onclick event listener */
export const addRemoveLevelButtonEventListener = (level: number): void => {
  const removeButton = document.getElementById(
    `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${level}`
  );

  removeButton.addEventListener("click", removeLevel);
};

/** adds onclick event listener to all remove buttons */
export const addRemoveLevelButtonEventListenerAll = (): void => {
  const removeButtons = document.querySelectorAll(`.${REMOVE_LEVEL_BUTTON}`);
  removeButtons.forEach((button) =>
    button.addEventListener("click", removeLevel)
  );
};

/**
 * Generate Remove button HTML. On-click handler should be added
 * seprately
 */
export const getRemoveButtonHTML = (level: number): HTMLLabelElement => {
  log.log(`[generating HTML getHtml]level: ${level}`);

  const btn = document.createElement("input");
  btn.className = CONST.REMOVE_LEVEL_BUTTON_CLASS;
  btn.type = "button";
  const id = `${CONST.REMOVE_LEVEL_BUTTON_ID_WITHOUT_LEVEL}${level}`;
  btn.name = id;
  btn.id = id;
  btn.value = "x";

  const removeLabel = document.createElement("label");
  removeLabel.className = CONST.REMOVE_LEVEL_LABEL_CLASS;
  const removeLabelText = document.createTextNode("RemoveLevel");
  removeLabel.appendChild(removeLabelText);
  removeLabel.appendChild(btn);
  removeLabel.htmlFor = id;

  return removeLabel;
};
