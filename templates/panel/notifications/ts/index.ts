// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
