// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * querySelector is the the selector that will be
 * used to fetch elements.
 * So when using class-names, pass in ".whatever-classname"
 * and for ID, "#id".
 * */
const getFormUrl = (querySelector?: string | HTMLFormElement): string => {
  let form;
  if (querySelector === undefined) {
    form = <HTMLFormElement>document.querySelector("form");
  }
  if (typeof querySelector == "string" || querySelector instanceof String) {
    form = <HTMLFormElement>document.querySelector(querySelector.toString());
  }
  if (querySelector instanceof HTMLFormElement) {
    form = querySelector;
  }

  if (form !== undefined && form !== null) {
    return form.action;
  } else {
    throw new Error("Can't find form");
  }
};

export default getFormUrl;
