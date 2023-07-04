// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import getFormUrl from "../../../../utils/getFormUrl";
import genJsonPayload from "../../../../utils/genJsonPayload";
import createError from "../../../../components/error";

import VIEWS from "../../../../views/v1/routes";

import { validate, FORM } from "../../add/novice/ts/form";

const SUBMIT_BTN = <HTMLButtonElement>(
  document.querySelector(".sitekey-form__submit")
);
const submit = async (e: Event) => {
  e.preventDefault();

  const key = SUBMIT_BTN.dataset.sitekey;
  const formUrl = getFormUrl(FORM);
  const payload = {
    pattern: validate(e),
    key,
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

const addSubmitEventListener = (): void =>
  FORM.addEventListener("submit", submit, true);

export default addSubmitEventListener;
