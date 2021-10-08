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
