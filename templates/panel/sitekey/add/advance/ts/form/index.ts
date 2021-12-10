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

import { LEVELS } from "../levels";

import getFormUrl from "../../../../../../utils/getFormUrl";
import genJsonPayload from "../../../../../../utils/genJsonPayload";

import VIEWS from "../../../../../../views/v1/routes";

import validateDescription from "./validateDescription";
import validateDuration from "./validateDuration";

import createError from "../../../../../../components/error";

export const SITE_KEY_FORM_CLASS = "sitekey-form";
export const FORM = <HTMLFormElement>(
  document.querySelector(`.${SITE_KEY_FORM_CLASS}`)
);

export const addSubmitEventListener = (): void =>
  FORM.addEventListener("submit", submit, true);

const submit = async (e: Event) => {
  e.preventDefault();

  const description = validateDescription(e);
  const duration = validateDuration();

  const formUrl = getFormUrl(FORM);

  const levels = LEVELS.getLevels();
  console.debug(`[form submition]: levels: ${levels}`);

  const payload = {
    levels: levels,
    duration,
    description,
  };

  console.debug(`[form submition] json payload: ${JSON.stringify(payload)}`);

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    const data = await res.json();
    window.location.assign(VIEWS.viewSitekey(data.key));
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

export default addSubmitEventListener;
