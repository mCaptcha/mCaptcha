// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

export const ERR_CONTAINER_ID = "err__container";
export const ERR_MSG_CONTAINER = "err__msg-container"; // class
export const ERR_CLOSE = "err__close"; // class

export const DEFAULT_LIFETIME = 5000;

const err = () => {
  let element;
  return (() => {
    if (element === undefined) {
      element = document.getElementById(ERR_CONTAINER_ID);
    }
    return element;
  })();
};

/**
 * create error message
 *
 * @param {string|HTMLElement} message: an error message
 * @param {number} lifetime: duration in milliseconds after which error
 * message will be deleted
 */
const createError = (
  message: string | HTMLElement,
  lifetime: number = DEFAULT_LIFETIME,
): void => {
  const box = document.createElement("div");

  const msg = () => {
    if (typeof message === "string") {
      return document.createTextNode(message);
    } else {
      return message;
    }
  };

  box.className = ERR_MSG_CONTAINER;
  box.appendChild(msg());

  const deleteBtn = document.createElement("button");
  const deleteMsg = document.createTextNode("x");
  deleteBtn.appendChild(deleteMsg);
  deleteBtn.className = ERR_CLOSE;
  box.appendChild(deleteBtn);

  err().appendChild(box);

  const timer = setTimeout(() => box.remove(), lifetime);

  const deleteHandler = (e: Event) => {
    e.preventDefault();
    window.clearTimeout(timer);
    box.remove();
  };

  deleteBtn.addEventListener("click", e => deleteHandler(e));
};

export default createError;
