/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

import genJsonPayload from "../../../utils/genJsonPayload";
import createError from "../../../components/error";

import ROUTES from "../../../api/v1/routes";

const BTN = document.querySelectorAll(".notification__mark-read-btn");
const TABLE_BODY = document.querySelector(".notification__body");

const notification_record = (id: number) =>
  <HTMLElement>TABLE_BODY.querySelector(`#notification__item-${id}`);

const markRead = async (e: Event) => {
  const element = <HTMLElement>e.target;

  const id = Number.parseInt(element.dataset.id);

  const payload = {
    id,
  };

  const res = await fetch(ROUTES.markNotificationRead, genJsonPayload(payload));
  if (res.ok) {
    notification_record(id).remove();
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

const addMarkReadEventListenet = () => {
  BTN.forEach(btn => {
    btn.addEventListener("click", markRead, true);
  });
};

export const index = (): void => {
  addMarkReadEventListenet();
};
