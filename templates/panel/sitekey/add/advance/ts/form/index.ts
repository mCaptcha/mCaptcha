// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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


  const PUBLISH_BENCHMARKS = <HTMLInputElement>(
    FORM.querySelector("#publish_benchmarks")
  );



  const description = validateDescription(e);
  const duration = validateDuration();

  const formUrl = getFormUrl(FORM);

  const levels = LEVELS.getLevels();
  console.debug(`[form submition]: levels: ${levels}`);

  const payload = {
    levels: levels,
    duration,
    description,
    publish_benchmarks: PUBLISH_BENCHMARKS.checked,
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
