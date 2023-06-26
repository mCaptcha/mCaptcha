// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import isBlankString from "../../../../../../utils/isBlankString";

const validateDescription = (e: Event): string => {
  const inputElement = <HTMLInputElement>document.getElementById("description");
  const val = inputElement.value;
  const filed = "Description";
  isBlankString(val, filed, e);
  return val;
};

export default validateDescription;
