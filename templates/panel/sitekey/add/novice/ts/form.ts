// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import getFormUrl from "../../../../../utils/getFormUrl";
import genJsonPayload from "../../../../../utils/genJsonPayload";
import isBlankString from "../../../../../utils/isBlankString";
import isNumber from "../../../../../utils/isNumber";

import VIEWS from "../../../../../views/v1/routes";

import validateDescription from "../../advance/ts/form/validateDescription";

import createError from "../../../../../components/error";

export const SITE_KEY_FORM_CLASS = "sitekey-form";
export const FORM = <HTMLFormElement>(
  document.querySelector(`.${SITE_KEY_FORM_CLASS}`)
);

export const addSubmitEventListener = (): void =>
  FORM.addEventListener("submit", submit, true);

export const break_my_site_name = "traffic that broke your website";
export const avg_traffic_name = "average";
export const peak_traffic_name = "maximum traffic your website can handle";

type TrafficPattern = {
  avg_traffic: number;
  peak_sustainable_traffic: number;
  broke_my_site_traffic?: number;
  description: string;
  publish_benchmarks: boolean;
};

export const validate = (e: Event): TrafficPattern => {
  const description = validateDescription(e);

  let broke_is_set = false;

  const AVG_TRAFFIC = <HTMLInputElement>FORM.querySelector("#avg_traffic");
  const PEAK_TRAFFIC = <HTMLInputElement>(
    FORM.querySelector("#peak_sustainable_traffic")
  );
  const BROKE_MY_SITE_TRAFFIC = <HTMLInputElement>(
    FORM.querySelector("#broke_my_site_traffic")
  );

  const PUBLISH_BENCHMARKS = <HTMLInputElement>(
    FORM.querySelector("#publish_benchmarks")
  );

  isBlankString(AVG_TRAFFIC.value, avg_traffic_name);
  isBlankString(PEAK_TRAFFIC.value, peak_traffic_name);

  const numberCheck = (name: string, field: HTMLInputElement) => {
    if (!isNumber(field.value)) {
      createError(`${name} must be a number`);
      throw new Error(`${name} must be a number`);
    }
    return true;
  };

  numberCheck(avg_traffic_name, AVG_TRAFFIC);
  numberCheck(peak_traffic_name, PEAK_TRAFFIC);
  if (BROKE_MY_SITE_TRAFFIC.value.trim().length > 0) {
    numberCheck(break_my_site_name, BROKE_MY_SITE_TRAFFIC);
    broke_is_set = true;
  }

  const avg_traffic = Number.parseInt(AVG_TRAFFIC.value);
  const peak_sustainable_traffic = Number.parseInt(PEAK_TRAFFIC.value);
  let broke_my_site_traffic = null;

  const mustBeGreater = (lhs: string, rhs: string) => {
    const msg = `${lhs} must be greater than ${rhs}`;
    createError(msg);
    throw new Error(msg);
  };

  if (avg_traffic >= peak_sustainable_traffic) {
    mustBeGreater(peak_traffic_name, avg_traffic_name);
  } else if (broke_is_set) {
    broke_my_site_traffic = Number.parseInt(BROKE_MY_SITE_TRAFFIC.value);
    if (peak_sustainable_traffic >= broke_my_site_traffic) {
      mustBeGreater(break_my_site_name, peak_traffic_name);
    }
  }

  const payload = {
    avg_traffic,
    peak_sustainable_traffic,
    broke_my_site_traffic,
    description,
    publish_benchmarks: PUBLISH_BENCHMARKS.checked,
  };

  return payload;
};

const submit = async (e: Event) => {
  e.preventDefault();

  const formUrl = getFormUrl(FORM);
  const payload = validate(e);
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
