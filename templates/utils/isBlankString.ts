// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import createError from "../components/error/";

const isBlankString = (value: string|number, field: string, event?: Event): boolean => {
  value = value.toString();
  if (!value.replace(/\s/g, "").length) {
    if (event !== undefined) {
      event.preventDefault();
    }
    const msg = `${field} can't be empty`;
    createError(msg);
    throw new  Error(msg);
  }
  return false;
};

export default isBlankString;
