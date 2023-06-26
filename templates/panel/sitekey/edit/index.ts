// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import * as Add from "../add/advance/ts/form/";
import addLevelButtonAddEventListener from "../add/advance/ts/addLevelButton";
import { addRemoveLevelButtonEventListenerAll } from "../add/advance/ts/removeLevelButton";
import getNumLevels from "../add/advance/ts/levels/getNumLevels";
import validateLevel from "../add/advance/ts/levels/validateLevel";
import * as UpdateLevel from "../add/advance/ts/levels/updateLevel";
import validateDescription from "../add/advance/ts/form/validateDescription";
import validateDuration from "../add/advance/ts/form/validateDuration";
import { LEVELS } from "../add/advance/ts/levels";

import getFormUrl from "../../../utils/getFormUrl";
import genJsonPayload from "../../../utils/genJsonPayload";
import createError from "../../../components/error";
import LazyElement from "../../../utils/lazyElement";

import VIEWS from "../../../views/v1/routes";

const BTN_ID = "sitekey-form__submit";
const BTN = new LazyElement(BTN_ID);

const submit = async (e: Event) => {
  e.preventDefault();

  const description = validateDescription(e);
  const duration = validateDuration();

  const formUrl = getFormUrl(Add.FORM);

  const levels = LEVELS.getLevels();
  console.debug(`[form submition]: levels: ${levels}`);

  const key = BTN.get().dataset.sitekey;


  const PUBLISH_BENCHMARKS = <HTMLInputElement>(
    Add.FORM.querySelector("#publish_benchmarks")
  );



  const payload = {
    levels,
    duration,
    description,
    key,
    publish_benchmarks: PUBLISH_BENCHMARKS.checked,
  };

  console.debug(`[form submition] json payload: ${JSON.stringify(payload)}`);

  const res = await fetch(formUrl, genJsonPayload(payload));
  if (res.ok) {
    window.location.assign(VIEWS.viewSitekey(key));
  } else {
    const err = await res.json();
    createError(err.error);
  }
};

const addSubmitEventListener = () => {
  Add.FORM.addEventListener("submit", submit, true);
};

const bootstrapLevels = () => {
  const levels = getNumLevels();
  addRemoveLevelButtonEventListenerAll();
  for (let i = 1; i <= levels - 1; i++) {
    validateLevel(i);
    UpdateLevel.register(i);
  }
};

export const index = (): void => {
  addSubmitEventListener();
  addLevelButtonAddEventListener();
  bootstrapLevels();
};
